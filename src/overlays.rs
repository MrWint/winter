use std::collections::HashMap;

const PAGE_SIZE: usize = 512;
const PARAGRAPH_SIZE: usize = 16;
const WINTER_NUM_OVERLAYS: usize = 16;
const SUMMER_NUM_OVERLAYS: usize = 20;


// inlines all overlays into a single executable
#[allow(dead_code)]
fn inline_overlays_summer() {
  let mut code = Vec::new();
  let mut overlay_relocations = Vec::new();
  let mut overlay_total_paragraphs = 0;
  let mut overlay_segment = HashMap::new();
  for oi in 1..=SUMMER_NUM_OVERLAYS {
    let cod = std::fs::read(format!("summeroverlay/OVL{}.COD", oi)).unwrap();
    let rel = std::fs::read(format!("summeroverlay/OVL{}.REL", oi)).unwrap();

    overlay_segment.insert(oi, overlay_total_paragraphs as u16);
    let overlay_paragraphs = cod.len().div_ceil(PARAGRAPH_SIZE);
    let padding = overlay_paragraphs * PARAGRAPH_SIZE - cod.len();
    code.extend_from_slice(&cod);
    code.append(&mut vec![0; padding]);

    let num_rel = u16::from_le_bytes(rel[0..2].try_into().unwrap()) as usize;
    for i in 0..num_rel {
      let index = u16::from_le_bytes(rel[2+4*i+0..2+4*i+2].try_into().unwrap()) as usize;
      assert!(index < overlay_paragraphs * PARAGRAPH_SIZE - padding);
      overlay_relocations.push((overlay_total_paragraphs, overlay_total_paragraphs * PARAGRAPH_SIZE + index));
    }

    overlay_total_paragraphs += overlay_paragraphs;
    assert!(code.len() == overlay_total_paragraphs * PARAGRAPH_SIZE);
  }

  println!("total relocation: {:x}", overlay_total_paragraphs);
  let relocation_paragraphs = overlay_total_paragraphs;

  let buf = std::fs::read("summeroverlay/UNSUMMER.EXE").unwrap();
  assert!(std::str::from_utf8(&buf[0..2]).unwrap() == "MZ");
  let last_page_bytes = u16::from_le_bytes(buf[2..4].try_into().unwrap()) as usize;
  let page_count = u16::from_le_bytes(buf[4..6].try_into().unwrap()) as usize;
  let binary_size = if last_page_bytes == 0 { page_count * PAGE_SIZE } else { (page_count - 1) * PAGE_SIZE + last_page_bytes };

  let relocation_items = u16::from_le_bytes(buf[6..8].try_into().unwrap()) as usize;
  let header_paragraphs = u16::from_le_bytes(buf[8..10].try_into().unwrap()) as usize;
  let header_size = header_paragraphs * PARAGRAPH_SIZE;
  let code_size = binary_size - header_size;
  // let minimum_allocation = u16::from_le_bytes(buf[10..12].try_into().unwrap());
  // let maximum_allocation = u16::from_le_bytes(buf[12..14].try_into().unwrap());
  let initial_ss = u16::from_le_bytes(buf[14..16].try_into().unwrap());
  // let initial_sp = u16::from_le_bytes(buf[16..18].try_into().unwrap());
  // let checksum = u16::from_le_bytes(buf[18..20].try_into().unwrap());
  // let initial_ip = u16::from_le_bytes(buf[20..22].try_into().unwrap());
  let initial_cs = u16::from_le_bytes(buf[22..24].try_into().unwrap());
  let relocation_table = u16::from_le_bytes(buf[24..26].try_into().unwrap()) as usize;
  // let overlay = u16::from_le_bytes(buf[26..28].try_into().unwrap());

  code.extend_from_slice(&buf[header_size..header_size + code_size]);

  for i in 0..relocation_items {
    let offset = u16::from_le_bytes(buf[relocation_table+4*i+0..relocation_table+4*i+2].try_into().unwrap()) as usize;
    let segment = u16::from_le_bytes(buf[relocation_table+4*i+2..relocation_table+4*i+4].try_into().unwrap()) as usize;
    let segment = segment + relocation_paragraphs;
    let index = segment * PARAGRAPH_SIZE + offset;
    overlay_relocations.push((relocation_paragraphs, index));
  }

  // adjust all existing relocation positions
  for &(overlay_paragraph, index) in overlay_relocations.iter() {
    let cur_value = u16::from_le_bytes(code[index..index+2].try_into().unwrap());
    if overlay_paragraph < relocation_paragraphs && cur_value >= 0x1849 && cur_value < 0x1849 + 0x517 {
      code[index..index+2].copy_from_slice(&(overlay_paragraph as u16).to_le_bytes());
    } else {
      code[index..index+2].copy_from_slice(&(cur_value + (relocation_paragraphs as u16)).to_le_bytes());
    }
  }

  // rewrite interrupt 3f as far call, add new relocations
  let overlay_file_index_table = [0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x03, 0x03, 0x03, 0x03, 0x04, 0x05, 0x05, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0B, 0x0C, 0x0D, 0x0D, 0x0D, 0x0D, 0x0E, 0x0E, 0x0E, 0x0E, 0x0F, 0x0F, 0x0F, 0x0F, 0x10, 0x10, 0x10, 0x10, 0x11, 0x11, 0x11, 0x11, 0x12, 0x12, 0x12, 0x12, 0x13, 0x13, 0x13, 0x13, 0x14, 0x14, 0x14, 0x14];
  let mut replacement_count = 0;
  for i in 0..code.len()-4 {
    if code[i] != 0xCD || code[i+1] != 0x3f { continue; }
    let overlay_index = code[i+2] as usize;
    let overlay_segment = overlay_segment[&overlay_file_index_table[overlay_index]];
    let overlay_offset = u16::from_le_bytes(code[i+3..i+5].try_into().unwrap());
    code[i] = 0x9A;
    code[i+1..i+3].copy_from_slice(&overlay_offset.to_le_bytes());
    code[i+3..i+5].copy_from_slice(&overlay_segment.to_le_bytes());
    overlay_relocations.push((0, i+3));
    replacement_count += 1;
  }
  println!("performed {} replacements", replacement_count);
  overlay_relocations.sort();

  let mut result = Vec::new();
  result.extend_from_slice(&buf[0..28]);
  for &(_, index) in overlay_relocations.iter() {
    let segment = index / PARAGRAPH_SIZE;
    let offset = index % PARAGRAPH_SIZE;
    result.extend_from_slice(&(offset as u16).to_le_bytes());
    result.extend_from_slice(&(segment as u16).to_le_bytes());
  }
  let header_paragraphs = result.len().div_ceil(PARAGRAPH_SIZE);
  result.append(&mut vec![0; header_paragraphs * PARAGRAPH_SIZE - result.len()]);

  result.extend_from_slice(&code);

  let new_binary_size = result.len();
  let last_page_bytes = new_binary_size % PAGE_SIZE;
  result[2..4].copy_from_slice(&(last_page_bytes as u16).to_le_bytes());
  let page_count = new_binary_size.div_ceil(PAGE_SIZE);
  result[4..6].copy_from_slice(&(page_count as u16).to_le_bytes());
  result[6..8].copy_from_slice(&(overlay_relocations.len() as u16).to_le_bytes());
  result[8..10].copy_from_slice(&(header_paragraphs as u16).to_le_bytes());
  let initial_ss = initial_ss + relocation_paragraphs as u16;
  result[14..16].copy_from_slice(&initial_ss.to_le_bytes());
  let initial_cs = initial_cs + relocation_paragraphs as u16;
  result[22..24].copy_from_slice(&initial_cs.to_le_bytes());

  std::fs::write("summerout/RESUMMER.EXE", &result).unwrap();

  for oi in 1..=SUMMER_NUM_OVERLAYS {
    let mut cod = std::fs::read(format!("summeroverlay/OVL{}.COD", oi)).unwrap();
    let mut rel = std::fs::read(format!("summeroverlay/OVL{}.REL", oi)).unwrap();

    let num_rel = u16::from_le_bytes(rel[0..2].try_into().unwrap()) as usize;
    for i in 0..num_rel {
      let index = u16::from_le_bytes(rel[2+4*i+0..2+4*i+2].try_into().unwrap()) as usize;
      let cur_value = u16::from_le_bytes(cod[index..index+2].try_into().unwrap());
      cod[index..index+2].copy_from_slice(&(cur_value + (relocation_paragraphs as u16)).to_le_bytes());

      let cur_segment = u16::from_le_bytes(rel[2+4*i+2..2+4*i+4].try_into().unwrap());
      rel[2+4*i+2..2+4*i+4].copy_from_slice(&(cur_segment + (relocation_paragraphs as u16)).to_le_bytes());
    }

    std::fs::write(format!("summerout/OVL{}.COD", oi), cod).unwrap();
    std::fs::write(format!("summerout/OVL{}.REL", oi), rel).unwrap();
  }
}



// inlines all overlays into a single executable
#[allow(dead_code)]
fn inline_overlays_winter() {
  let mut code = Vec::new();
  let mut overlay_relocations = Vec::new();
  let mut overlay_total_paragraphs = 0;
  let mut overlay_segment = HashMap::new();
  for oi in 1..=WINTER_NUM_OVERLAYS {
    let cod = std::fs::read(format!("overlay/OVL{}.COD", oi)).unwrap();
    let rel = std::fs::read(format!("overlay/OVL{}.REL", oi)).unwrap();

    overlay_segment.insert(oi, overlay_total_paragraphs as u16);
    let overlay_paragraphs = cod.len().div_ceil(PARAGRAPH_SIZE);
    let padding = overlay_paragraphs * PARAGRAPH_SIZE - cod.len();
    code.extend_from_slice(&cod);
    code.append(&mut vec![0; padding]);

    let num_rel = u16::from_le_bytes(rel[0..2].try_into().unwrap()) as usize;
    for i in 0..num_rel {
      let index = u16::from_le_bytes(rel[2+4*i+0..2+4*i+2].try_into().unwrap()) as usize;
      assert!(index < overlay_paragraphs * PARAGRAPH_SIZE - padding);
      overlay_relocations.push((overlay_total_paragraphs, overlay_total_paragraphs * PARAGRAPH_SIZE + index));
    }

    overlay_total_paragraphs += overlay_paragraphs;
    assert!(code.len() == overlay_total_paragraphs * PARAGRAPH_SIZE);
  }

  println!("total relocation: {:x}", overlay_total_paragraphs);
  let relocation_paragraphs = overlay_total_paragraphs;

  let buf = std::fs::read("overlay/UNWINTER.EXE").unwrap();
  assert!(std::str::from_utf8(&buf[0..2]).unwrap() == "MZ");
  let last_page_bytes = u16::from_le_bytes(buf[2..4].try_into().unwrap()) as usize;
  let page_count = u16::from_le_bytes(buf[4..6].try_into().unwrap()) as usize;
  let binary_size = if last_page_bytes == 0 { page_count * PAGE_SIZE } else { (page_count - 1) * PAGE_SIZE + last_page_bytes };

  let relocation_items = u16::from_le_bytes(buf[6..8].try_into().unwrap()) as usize;
  let header_paragraphs = u16::from_le_bytes(buf[8..10].try_into().unwrap()) as usize;
  let header_size = header_paragraphs * PARAGRAPH_SIZE;
  let code_size = binary_size - header_size;
  // let minimum_allocation = u16::from_le_bytes(buf[10..12].try_into().unwrap());
  // let maximum_allocation = u16::from_le_bytes(buf[12..14].try_into().unwrap());
  let initial_ss = u16::from_le_bytes(buf[14..16].try_into().unwrap());
  // let initial_sp = u16::from_le_bytes(buf[16..18].try_into().unwrap());
  // let checksum = u16::from_le_bytes(buf[18..20].try_into().unwrap());
  // let initial_ip = u16::from_le_bytes(buf[20..22].try_into().unwrap());
  let initial_cs = u16::from_le_bytes(buf[22..24].try_into().unwrap());
  let relocation_table = u16::from_le_bytes(buf[24..26].try_into().unwrap()) as usize;
  // let overlay = u16::from_le_bytes(buf[26..28].try_into().unwrap());

  code.extend_from_slice(&buf[header_size..header_size + code_size]);

  for i in 0..relocation_items {
    let offset = u16::from_le_bytes(buf[relocation_table+4*i+0..relocation_table+4*i+2].try_into().unwrap()) as usize;
    let segment = u16::from_le_bytes(buf[relocation_table+4*i+2..relocation_table+4*i+4].try_into().unwrap()) as usize;
    let segment = segment + relocation_paragraphs;
    let index = segment * PARAGRAPH_SIZE + offset;
    overlay_relocations.push((relocation_paragraphs, index));
  }

  // adjust all existing relocation positions
  for &(overlay_paragraph, index) in overlay_relocations.iter() {
    let cur_value = u16::from_le_bytes(code[index..index+2].try_into().unwrap());
    if overlay_paragraph < relocation_paragraphs && cur_value >= 0x1849 && cur_value < 0x1849 + 0x517 {
      code[index..index+2].copy_from_slice(&(overlay_paragraph as u16).to_le_bytes());
    } else {
      code[index..index+2].copy_from_slice(&(cur_value + (relocation_paragraphs as u16)).to_le_bytes());
    }
  }

  // rewrite interrupt 3f as far call, add new relocations
  let overlay_file_index_table = [0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 9, 9, 0xA, 0xA, 0xA, 0xA, 0xA, 0xA, 0xA, 0xA, 0xA, 0xA, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0xF, 0x10];
  let mut replacement_count = 0;
  for i in 0..code.len()-4 {
    if code[i] != 0xCD || code[i+1] != 0x3f { continue; }
    let overlay_index = code[i+2] as usize;
    let overlay_segment = overlay_segment[&overlay_file_index_table[overlay_index]];
    let overlay_offset = u16::from_le_bytes(code[i+3..i+5].try_into().unwrap());
    code[i] = 0x9A;
    code[i+1..i+3].copy_from_slice(&overlay_offset.to_le_bytes());
    code[i+3..i+5].copy_from_slice(&overlay_segment.to_le_bytes());
    overlay_relocations.push((0, i+3));
    replacement_count += 1;
  }
  println!("performed {} replacements", replacement_count);
  overlay_relocations.sort();

  let mut result = Vec::new();
  result.extend_from_slice(&buf[0..28]);
  for &(_, index) in overlay_relocations.iter() {
    let segment = index / PARAGRAPH_SIZE;
    let offset = index % PARAGRAPH_SIZE;
    result.extend_from_slice(&(offset as u16).to_le_bytes());
    result.extend_from_slice(&(segment as u16).to_le_bytes());
  }
  let header_paragraphs = result.len().div_ceil(PARAGRAPH_SIZE);
  result.append(&mut vec![0; header_paragraphs * PARAGRAPH_SIZE - result.len()]);

  result.extend_from_slice(&code);

  let new_binary_size = result.len();
  let last_page_bytes = new_binary_size % PAGE_SIZE;
  result[2..4].copy_from_slice(&(last_page_bytes as u16).to_le_bytes());
  let page_count = new_binary_size.div_ceil(PAGE_SIZE);
  result[4..6].copy_from_slice(&(page_count as u16).to_le_bytes());
  result[6..8].copy_from_slice(&(overlay_relocations.len() as u16).to_le_bytes());
  result[8..10].copy_from_slice(&(header_paragraphs as u16).to_le_bytes());
  let initial_ss = initial_ss + relocation_paragraphs as u16;
  result[14..16].copy_from_slice(&initial_ss.to_le_bytes());
  let initial_cs = initial_cs + relocation_paragraphs as u16;
  result[22..24].copy_from_slice(&initial_cs.to_le_bytes());

  std::fs::write("out/REWINTER.EXE", &result).unwrap();

  for oi in 1..=WINTER_NUM_OVERLAYS {
    let mut cod = std::fs::read(format!("overlay/OVL{}.COD", oi)).unwrap();
    let mut rel = std::fs::read(format!("overlay/OVL{}.REL", oi)).unwrap();

    let num_rel = u16::from_le_bytes(rel[0..2].try_into().unwrap()) as usize;
    for i in 0..num_rel {
      let index = u16::from_le_bytes(rel[2+4*i+0..2+4*i+2].try_into().unwrap()) as usize;
      let cur_value = u16::from_le_bytes(cod[index..index+2].try_into().unwrap());
      cod[index..index+2].copy_from_slice(&(cur_value + (relocation_paragraphs as u16)).to_le_bytes());

      let cur_segment = u16::from_le_bytes(rel[2+4*i+2..2+4*i+4].try_into().unwrap());
      rel[2+4*i+2..2+4*i+4].copy_from_slice(&(cur_segment + (relocation_paragraphs as u16)).to_le_bytes());
    }

    std::fs::write(format!("out/OVL{}.COD", oi), cod).unwrap();
    std::fs::write(format!("out/OVL{}.REL", oi), rel).unwrap();
  }
}
