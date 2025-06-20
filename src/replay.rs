use crate::skijumpsimple::{AnimationAction, AnimationState, Fixed16, InputFrame, Mem, SkiJumpState, Vec3Fixed16};

#[derive(Debug)]
pub struct PlayerData {
  color_02: u8, // 0 = gray, 1 = blue, 2 = red, 3 = yellow, 4 = pink, 5 = red+blue, 6 = purple+blue, 7 = red-gray, 8 = pink+gray, 9 = gray+orange, 10=green+red, 11=blue+yellow, 12=red+yellow, 13 = gray+orange, 14 = yellow+red, 15 = gray+blue
  unknown_18_always_5: u8,
  unknown_1e: u8
}
impl PlayerData {
  fn from_replay(data: &[u8]) -> PlayerData {
    PlayerData {
      color_02: data[0x2],
      unknown_18_always_5: data[0x18],
      unknown_1e: data[0x1e],
    }
  }
  pub fn synthetic() -> PlayerData {
    PlayerData {
      color_02: 10,
      unknown_18_always_5: 5,
      unknown_1e: 0x1b,
    }
  }
  fn to_replay(&self) -> Vec<u8> {
    let mut result = vec![0; 0x42];
    result[0x2] = self.color_02;
    result[0x18] = self.unknown_18_always_5;
    result[0x1e] = self.unknown_1e;
    result
  }
}

#[derive(Clone, Copy, Debug)]
pub struct MemData {
  counter_529f6: u16,
  counter_529f8: u16,
  ski_flight_angle: i16,
}
impl MemData {
  pub fn synthetic(counter_529f6: u16, counter_529f8: u16, ski_flight_angle: i16) -> MemData {
    MemData {
      counter_529f6,
      counter_529f8,
      ski_flight_angle,
    }
  }
  fn from_replay(buf: &[u8]) -> MemData {
    let counter_529f6 = u16::from_le_bytes(buf[0..2].try_into().unwrap());
    let counter_529f8 = u16::from_le_bytes(buf[2..4].try_into().unwrap());

    assert_eq!(0, u16::from_le_bytes(buf[4..6].try_into().unwrap()));  // ski_jump_frame_counter_503F8
    assert_eq!(0, i32::from_le_bytes(buf[6..0xa].try_into().unwrap()));  // dword_503FA.x
    assert_eq!(63, i32::from_le_bytes(buf[0xa..0xe].try_into().unwrap()));  // dword_503FA.y
    assert_eq!(-192, i32::from_le_bytes(buf[0xe..0x12].try_into().unwrap()));  // dword_503FA.z
    assert_eq!(0, i32::from_le_bytes(buf[0x12..0x16].try_into().unwrap()));  // position_vec.x
    assert_eq!(-1, i32::from_le_bytes(buf[0x16..0x1a].try_into().unwrap()));  // position_vec.y
    assert_eq!(0x1c0, i32::from_le_bytes(buf[0x1a..0x1e].try_into().unwrap()));  // position_vec.z
    assert_eq!(0, u16::from_le_bytes(buf[0x1e..0x20].try_into().unwrap()));  // word_50412
    assert_eq!(0x800, i16::from_le_bytes(buf[0x20..0x22].try_into().unwrap()));  // word_50414.x
    assert_eq!(0, i16::from_le_bytes(buf[0x22..0x24].try_into().unwrap()));  // word_50414.y
    assert_eq!(0, i16::from_le_bytes(buf[0x24..0x26].try_into().unwrap()));  // word_50414.z
    assert_eq!(0, i16::from_le_bytes(buf[0x26..0x28].try_into().unwrap()));  // word_5041a.x
    assert_eq!(0, i16::from_le_bytes(buf[0x28..0x2a].try_into().unwrap()));  // word_5041a.y
    assert_eq!(14, i16::from_le_bytes(buf[0x2a..0x2c].try_into().unwrap()));  // word_5041a.z
    assert_eq!(2, buf[0x2c]);  // ski_jump_animation_state_50420
    assert_eq!(0, buf[0x2d]);  // ski_jump_animation_state_counter_50421
    assert_eq!(0, u16::from_le_bytes(buf[0x2e..0x30].try_into().unwrap()));  // ski_jump_animation_randomness_maybe_50422
    assert_eq!(0, u16::from_le_bytes(buf[0x30..0x32].try_into().unwrap()));  // ski_jump_current_distance
    assert!(buf[0x32..0xa2].iter().all(|&v| v == 0)); // ski_jump_dust_particle_data_maybe_50426
    assert_eq!(0, u16::from_le_bytes(buf[0xa2..0xa4].try_into().unwrap()));  // ski_jump_displayed_velocity_50496
    assert_eq!(0, i32::from_le_bytes(buf[0xa4..0xa8].try_into().unwrap()));  // vec_50498.x
    assert_eq!(-1, i32::from_le_bytes(buf[0xa8..0xac].try_into().unwrap()));  // vec_50498.y
    assert_eq!(0x1c0, i32::from_le_bytes(buf[0xac..0xb0].try_into().unwrap()));  // vec_50498.z
    assert_eq!(0, buf[0xb0]);  // byte_504A4
    assert_eq!(0, buf[0xb1]);  // ski_jump_ending_turn_504A5
    let ski_flight_angle = i16::from_le_bytes(buf[0xb2..0xb4].try_into().unwrap());
    assert!(buf[0xb4..0x110].iter().all(|&v| v == 0)); // ski_jump_crash_ski_data_maybe_504A8
    assert_eq!(1, u16::from_le_bytes(buf[0x110..0x112].try_into().unwrap()));  // ski_jump_state_50504
    assert_eq!(0, u16::from_le_bytes(buf[0x112..0x114].try_into().unwrap()));  // ski_jump_state_framecount_50506
    assert_eq!(0, u16::from_le_bytes(buf[0x114..0x116].try_into().unwrap()));  // ski_jump_jmptrack_cur_segment_50508
    assert_eq!(0, u16::from_le_bytes(buf[0x116..0x118].try_into().unwrap()));  // word_5050A
    assert_eq!(0, i16::from_le_bytes(buf[0x118..0x11a].try_into().unwrap()));  // ski_jump_surface_normal_vec_5050C.x
    assert_eq!(0x7ffd, i16::from_le_bytes(buf[0x11a..0x11c].try_into().unwrap()));  // ski_jump_surface_normal_vec_5050C.y
    assert_eq!(0, i16::from_le_bytes(buf[0x11c..0x11e].try_into().unwrap()));  // ski_jump_surface_normal_vec_5050C.z
    assert_eq!(0, i16::from_le_bytes(buf[0x11e..0x120].try_into().unwrap()));  // vec_tmp_50512.x
    assert_eq!(0x7ffd, i16::from_le_bytes(buf[0x120..0x122].try_into().unwrap()));  // vec_tmp_50512.y
    assert_eq!(0, i16::from_le_bytes(buf[0x122..0x124].try_into().unwrap()));  // vec_tmp_50512.z
    assert_eq!(1, u16::from_le_bytes(buf[0x124..0x126].try_into().unwrap()));  // ski_jump_is_grounded_50518
    assert_eq!(0, u16::from_le_bytes(buf[0x126..0x128].try_into().unwrap()));  // word_5051A
    assert_eq!(3, buf[0x128]);  // ski_jump_animation_action_5051C
    assert_eq!(0, buf[0x129]);  // byte_5051D
    assert_eq!(0, i16::from_le_bytes(buf[0x12a..0x12c].try_into().unwrap()));  // ski_jump_raw_velocity_vec.x
    assert_eq!(0, i16::from_le_bytes(buf[0x12c..0x12e].try_into().unwrap()));  // ski_jump_raw_velocity_vec.y
    assert_eq!(0, i16::from_le_bytes(buf[0x12e..0x130].try_into().unwrap()));  // ski_jump_raw_velocity_vec.z
    assert_eq!(0, i16::from_le_bytes(buf[0x130..0x132].try_into().unwrap()));  // ski_jump_normalized_velocity_vec_50524.x
    assert_eq!(0, i16::from_le_bytes(buf[0x132..0x134].try_into().unwrap()));  // ski_jump_normalized_velocity_vec_50524.y
    assert_eq!(0, i16::from_le_bytes(buf[0x134..0x136].try_into().unwrap()));  // ski_jump_normalized_velocity_vec_50524.z
    assert_eq!(0, u16::from_le_bytes(buf[0x136..0x138].try_into().unwrap()));  // ski_jump_velocity_magnitude_5052A
    assert_eq!(1, u16::from_le_bytes(buf[0x138..0x13a].try_into().unwrap()));  // ski_jump_segment_square_5052C
    assert_eq!(0, u16::from_le_bytes(buf[0x13a..0x13c].try_into().unwrap()));  // ski_jump_upwards_movement_frames
    assert_eq!(0, u32::from_le_bytes(buf[0x13c..0x140].try_into().unwrap()));  // ski_jump_unk_dust_maybe_50530

    assert_eq!(1, u16::from_le_bytes(buf[0x140..0x142].try_into().unwrap()));  // word_50534
    assert_eq!(0x800, u16::from_le_bytes(buf[0x142..0x144].try_into().unwrap()));  // word_50536
    assert_eq!(0x7e00, u16::from_le_bytes(buf[0x144..0x146].try_into().unwrap()));  // word_50538
    assert_eq!(0, u16::from_le_bytes(buf[0x146..0x148].try_into().unwrap()));  // byte_5053A
    assert_eq!(0, i32::from_le_bytes(buf[0x148..0x14c].try_into().unwrap()));  // dword_5053C.x
    assert_eq!(-1, i32::from_le_bytes(buf[0x14c..0x150].try_into().unwrap()));  // dword_5053C.y
    assert_eq!(0x1c0, i32::from_le_bytes(buf[0x150..0x154].try_into().unwrap()));  // dword_5053C.z
    assert_eq!(0, i16::from_le_bytes(buf[0x154..0x156].try_into().unwrap()));  // word_50548.x
    assert_eq!(0, i16::from_le_bytes(buf[0x156..0x158].try_into().unwrap()));  // word_50548.y
    assert_eq!(0, i16::from_le_bytes(buf[0x158..0x15a].try_into().unwrap()));  // word_50548.z
    assert_eq!(0, i16::from_le_bytes(buf[0x15a..0x15c].try_into().unwrap()));  // word_5054E.x
    assert_eq!(0, i16::from_le_bytes(buf[0x15c..0x15e].try_into().unwrap()));  // word_5054E.y
    assert_eq!(0, i16::from_le_bytes(buf[0x15e..0x160].try_into().unwrap()));  // word_5054E.z
    assert_eq!(0x20, u16::from_le_bytes(buf[0x160..0x162].try_into().unwrap()));  // word_50554
    assert_eq!(0x280, u16::from_le_bytes(buf[0x162..0x164].try_into().unwrap()));  // word_50556
    assert_eq!(0, buf[0x164]);  // ski_jump_sub_state_maybe_50558
    assert_eq!(1, buf[0x165]);  // ski_jump_sub_state_frame_count_50559
    assert_eq!(0, buf[0x166]);  // ski_jump_never_lifted_off_flag
    assert_eq!(0, buf[0x167]);  // landing_delay_counter_5055B
    assert_eq!(0, u16::from_le_bytes(buf[0x168..0x16a].try_into().unwrap()));  // ski_jump_lr_random_deflection_5055C

    MemData {
      counter_529f6,
      counter_529f8,
      ski_flight_angle,
    }
  }

  fn to_replay(&self) -> Vec<u8> {
    let mut buf = vec![0; 0x16a];

    buf[0..2].copy_from_slice(&self.counter_529f6.to_le_bytes());
    buf[2..4].copy_from_slice(&self.counter_529f8.to_le_bytes());
    buf[0xb2..0xb4].copy_from_slice(&self.ski_flight_angle.to_le_bytes());

    buf[0xa] = 63;  // dword_503FA.y
    buf[0xe..0x12].copy_from_slice(&(-192_i32).to_le_bytes());  // dword_503FA.z
    buf[0x16..0x1a].copy_from_slice(&(-1_i32).to_le_bytes());  // position_vec.y
    buf[0x1a..0x1e].copy_from_slice(&(0x1c0_i32).to_le_bytes());  // position_vec.z
    buf[0x20..0x22].copy_from_slice(&(0x800_i16).to_le_bytes());  // word_50414.x
    buf[0x2a..0x2c].copy_from_slice(&(14_i16).to_le_bytes());  // word_5041a.z
    buf[0x2c] = 2;  // ski_jump_animation_state_50420
    buf[ 0xa8.. 0xac].copy_from_slice(&(    -1_i32).to_le_bytes());  // vec_50498.y
    buf[ 0xac.. 0xb0].copy_from_slice(&( 0x1c0_i32).to_le_bytes());  // vec_50498.z
    buf[0x110..0x112].copy_from_slice(&(     1_u16).to_le_bytes());  // ski_jump_state_50504
    buf[0x11a..0x11c].copy_from_slice(&(0x7ffd_i16).to_le_bytes());  // ski_jump_surface_normal_vec_5050C.y
    buf[0x120..0x122].copy_from_slice(&(0x7ffd_i16).to_le_bytes());  // vec_tmp_50512.y
    buf[0x124..0x126].copy_from_slice(&(     1_u16).to_le_bytes());  // ski_jump_is_grounded_50518
    buf[0x128] = 3;  // ski_jump_animation_action_5051C
    buf[0x138..0x13a].copy_from_slice(&(     1_u16).to_le_bytes());  // ski_jump_segment_square_5052C
    buf[0x140..0x142].copy_from_slice(&(     1_u16).to_le_bytes());  // word_50534
    buf[0x142..0x144].copy_from_slice(&( 0x800_u16).to_le_bytes());  // word_50536
    buf[0x144..0x146].copy_from_slice(&(0x7e00_u16).to_le_bytes());  // word_50538
    buf[0x14c..0x150].copy_from_slice(&(    -1_i32).to_le_bytes());  // dword_5053C.y
    buf[0x150..0x154].copy_from_slice(&( 0x1c0_i32).to_le_bytes());  // dword_5053C.z
    buf[0x160..0x162].copy_from_slice(&(  0x20_u16).to_le_bytes());  // word_50554
    buf[0x162..0x164].copy_from_slice(&( 0x280_u16).to_le_bytes());  // word_50556
    buf[0x165] = 1;  // ski_jump_sub_state_frame_count_50559

    buf
  }

  pub fn to_mem(&self) -> Mem {
    let mut mem = Mem::default();

    mem.counter_529f6 = self.counter_529f6;
    mem.counter_529f8 = self.counter_529f8;
    mem.ski_flight_angle = self.ski_flight_angle;

    mem.position_vec = Vec3Fixed16 { x: Fixed16::from_raw_i16(0), y: Fixed16::from_raw_i16(-1), z: Fixed16::from_raw_i16(0x1c0) };
    mem.animation_state = AnimationState::Start;
    mem.state = SkiJumpState::Jumping;
    mem.surface_normal_vec = Vec3Fixed16 { x: Fixed16::from_raw_i16(0), y: Fixed16::from_raw_i16(0x7ffd), z: Fixed16::from_raw_i16(0) };
    mem.is_grounded = true;
    mem.animation_action = AnimationAction::Duck;

    mem
  }
}

const STATE_START: usize = 0xe;
const STATE_SIZE: usize = 0x1ac;
const INPUTS_START: usize = STATE_START + STATE_SIZE;
const PLAYER_DATA_LEN: usize = 0x42;

#[derive(Debug)]
pub struct ReplayData {
  pub player_state: PlayerData,
  pub mem_state: MemData,
  pub inputs: Vec<InputFrame>,
}
impl ReplayData {
  #[allow(dead_code)]
  pub fn from_file(file_name: &str) -> ReplayData {
    let buf = std::fs::read(file_name).expect("failed to read replay file");
  
    let size_in_bytes = u16::from_le_bytes(buf[0..2].try_into().unwrap());
    let frame_count = u16::from_le_bytes(buf[2..4].try_into().unwrap());
    assert_eq!(STATE_START as u16, u16::from_le_bytes(buf[4..6].try_into().unwrap()));
    assert_eq!(STATE_SIZE as u16, u16::from_le_bytes(buf[6..8].try_into().unwrap()));
    assert_eq!(INPUTS_START as u16, u16::from_le_bytes(buf[8..10].try_into().unwrap()));
    let inputs_size_in_bytes = u16::from_le_bytes(buf[10..12].try_into().unwrap());
    let input_block_count = u16::from_le_bytes(buf[12..14].try_into().unwrap());
  
    assert_eq!(inputs_size_in_bytes, 4 * input_block_count);
    assert_eq!(size_in_bytes, INPUTS_START as u16 + inputs_size_in_bytes);
  
    fn parse_input_block(block: &[u8]) -> (usize, InputFrame) {
      let x = (block[0] & 7) as i8 - 3;
      let y = ((block[0] >> 3) & 7) as i8 - 3;
      let len = (block[2] & 0x3f) as usize;
      let b12 = (block[3] & 0x10) != 0;
      (len, InputFrame { x, y, enter: b12 })
    }
  
    let mut offset = 0;
    let mut i1 = (0, InputFrame::default());
    let mut i2 = (0, InputFrame::default());
    let mut inputs = Vec::new();
    while inputs.len() < frame_count as usize {
      if i1.0 == 0 {
        i1 = parse_input_block(&buf[INPUTS_START + 4*offset..INPUTS_START + 4*offset+4]);
        offset += 1;
      }
      i1.0 -= 1;
      if i2.0 == 0 {
        i2 = parse_input_block(&buf[INPUTS_START + 4*offset..INPUTS_START + 4*offset+4]);
        offset += 1;
      }
      i2.0 -= 1;
      inputs.push(if i2.1 == InputFrame::default() { i1.1 } else { i2.1 });
    }
  
    let player_state = PlayerData::from_replay(&buf[STATE_START..STATE_START + PLAYER_DATA_LEN]);
    let mem_state = MemData::from_replay(&buf[STATE_START + PLAYER_DATA_LEN..INPUTS_START]);
  
    ReplayData { player_state, mem_state, inputs }
  }

  #[allow(dead_code)]
  pub fn to_file(&self, file_name: &str) {
    fn to_input_block(len: usize, input: InputFrame) -> [u8; 4] {
      [(input.x + 3) as u8 | ((input.y + 3) << 3) as u8,
      0,
      (len & 0x3f) as u8,
      if input.enter { 0x10 } else { 0 }]
    }

    let mut inputs = Vec::new();
    let mut i1_written = 0;
    let mut i2_written = 0;

    while i1_written < self.inputs.len() {
      let mut i1len = 1;
      while i1len < 0x3f && i1_written+i1len < self.inputs.len() && self.inputs[i1_written+i1len] == self.inputs[i1_written] {
        i1len += 1;
      }
      inputs.extend_from_slice(&to_input_block(i1len, self.inputs[i1_written]));
      i1_written += i1len;
      if i2_written < i1_written {
        let i2len = std::cmp::min(0x3f, self.inputs.len() - i2_written);
        inputs.extend_from_slice(&to_input_block(i2len, InputFrame::default()));
        i2_written += i2len;
      }
    }

    let mut buf = Vec::with_capacity(INPUTS_START + inputs.len());
    buf.extend_from_slice(&((INPUTS_START + inputs.len()) as u16).to_le_bytes());  // size_in_bytes
    buf.extend_from_slice(&(self.inputs.len() as u16).to_le_bytes());  // frame_count
    buf.extend_from_slice(&(STATE_START as u16).to_le_bytes());
    buf.extend_from_slice(&(STATE_SIZE as u16).to_le_bytes());
    buf.extend_from_slice(&(INPUTS_START as u16).to_le_bytes());
    buf.extend_from_slice(&(inputs.len() as u16).to_le_bytes());  // inputs_size_in_bytes
    buf.extend_from_slice(&(inputs.len() as u16 / 4).to_le_bytes());  // input_block_count
    buf.extend_from_slice(&self.player_state.to_replay());
    buf.extend_from_slice(&self.mem_state.to_replay());
    buf.extend_from_slice(&inputs);

    std::fs::write(file_name, &buf).expect("failed to write replay file");
  }
}
