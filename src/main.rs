mod optimizer;
mod overlays;
mod replay;
mod rngtesting;
mod skijumpdecomp;
mod skijumpsimple;
mod unpack;

fn main() {
  optimizer::optimize();  // goal: 19716=113.7, 19725=113.8, 19735=113.9
}

// validates a replay against golden memory dumps
#[allow(dead_code)]
fn validate_replay() {
  replay::ReplayData::from_file("memdump/1058.RP5").to_file("TEST.RP5");
  let replay_data = replay::ReplayData::from_file("memdump/1058.RP5");
  let mut mem = replay_data.mem_state.to_mem();
  mem.compare_against_memdump("memdump/000.bin");
  for i in 0..168-2 {  // skip last two frames due to dust differences
    println!("{}", i);
    skijumpsimple::ski_jump_advance_frame_maybe_1f13e(&mut mem, replay_data.inputs[i]);
    mem.compare_against_memdump(&format!("memdump/{:03}.bin", i+1));
  }
  let mut mem = replay_data.mem_state.to_mem();
  let mut frame = 0;
  while !mem.has_failed && !mem.has_finished {
    skijumpsimple::ski_jump_advance_frame_maybe_1f13e(&mut mem, if replay_data.inputs.len() > frame { replay_data.inputs[frame] } else { skijumpsimple::InputFrame::default() });
    frame += 1;
  }
  println!("{} after {} frames with distance {}", if mem.has_finished { "finished" } else { "crashed" }, frame, mem.ski_jump_current_distance);
}

const PAGE_SIZE: usize = 512;

// extracts resources from the game executable
#[allow(dead_code)]
fn extract_resources(file_name: &str, _extraction_path: &str) {
  let buf = std::fs::read(file_name).unwrap();
  assert!(std::str::from_utf8(&buf[0..2]).unwrap() == "MZ");
  let last_page_bytes = u16::from_le_bytes(buf[2..4].try_into().unwrap()) as usize;
  let page_count = u16::from_le_bytes(buf[4..6].try_into().unwrap()) as usize;
  let resource_data_start = if last_page_bytes == 0 { page_count * PAGE_SIZE } else { (page_count - 1) * PAGE_SIZE + last_page_bytes };

  println!("Resources start at {:x}", resource_data_start);
  assert!(std::str::from_utf8(&buf[resource_data_start..resource_data_start+2]).unwrap() == "MB");
  let resource_count = u16::from_le_bytes(buf[resource_data_start+2..resource_data_start+4].try_into().unwrap()) as usize;
  for i in 0..resource_count {
    let length = u32::from_le_bytes(buf[resource_data_start+4+22*i+0..resource_data_start+4+22*i+4].try_into().unwrap()) as usize;
    let start_index = u32::from_le_bytes(buf[resource_data_start+4+22*i+4..resource_data_start+4+22*i+8].try_into().unwrap()) as usize;
    let name: Vec<u8> = buf[resource_data_start+4+22*i+8..resource_data_start+4+22*i+22].iter().cloned().take_while(|&b| b != 0).map(|b| b - 0x60).collect();
    let name = String::from_utf8(name).unwrap();
    println!("Resource {:x}: {} start {:x} end {:x} length {:x}", i, name, start_index, start_index+length, length);
    // std::fs::write(format!("{}/{}", extraction_path, name), &buf[start_index..start_index+length]).unwrap();
  }
}
