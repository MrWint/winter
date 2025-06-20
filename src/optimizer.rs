use crate::{replay::{MemData, PlayerData, ReplayData}, skijumpsimple::{flight_angle_correction, liftoff_input, rolling_adjustment, simulate_flight, ski_jump_advance_frame_maybe_1f13e, will_land_next_frame, AnimationState, InputFrame, Mem, SkiJumpState, SkiJumpSubState, Vec3Fixed16}};

// list of promising RNG seeds for maximum distance
#[allow(dead_code)]
const GOOD_RNG_VALUES: &[(u16, u16)] = &[
(0x38, 0x1f0), // 19719
(0x1e, 0x24b), // 19719
(0x9, 0x25e), // 19718
(0x3, 0x31c), // 19719
(0x2e, 0x454), // 19719
(0x2d, 0x549), // 19716
(0x1d, 0x8ca), // 19720
(0x31, 0xa43), // 19719
(0x11, 0xb1a), // 19719
(0x9, 0xb26), // 19721
(0x7, 0xb36), // 19720
(0x12, 0xbc3), // 19719
(0xe, 0xcbb), // 19719
(0x1f, 0xd27), // 19715
(0x29, 0xda6), // 19719
(0x23, 0xe41), // 19719
(0x13, 0x1177), // 19718
(0x7, 0x142c), // 19718
(0x8, 0x16d1), // 19720
(0x16, 0x17c4), // 19719
(0x23, 0x1834), // 19719
(0x3, 0x18ce), // 19719
(0x38, 0x1a16), // 19719
(0x3c, 0x1cce), // 19716
(0x2a, 0x1da3), // 19719
(0x2e, 0x1da6), // 19716
(0x1a, 0x1f77), // 19720
(0x32, 0x1f96), // 19719
(0x18, 0x1ff7), // 19720
(0x5, 0x2004), // 19719
(0x33, 0x2013), // 19719
(0x5, 0x2052), // 19715
(0x39, 0x209d), // 19719
(0x3a, 0x218f), // 19719
(0x19, 0x21d8), // 19719
(0x8, 0x2209), // 19719
(0xd, 0x22fd), // 19720
(0x12, 0x240c), // 19719
(0x28, 0x2462), // 19719
(0x0, 0x246d), // 19719
(0x22, 0x257b), // 19718
(0x39, 0x259f), // 19719
(0x2d, 0x2654), // 19719
(0x38, 0x2727), // 19719
(0x22, 0x27a2), // 19719
(0x21, 0x27f8), // 19718
(0x2c, 0x2845), // 19718
(0x5, 0x28c1), // 19719
(0x1, 0x2a1f), // 19720
(0x30, 0x2a53), // 19720
(0x31, 0x2b2e), // 19719
(0x27, 0x2d00), // 19719
(0x3e, 0x2d33), // 19719
(0x37, 0x2e4e), // 19715
(0x2f, 0x2ea5), // 19719
(0x17, 0x2edf), // 19720
(0x1f, 0x2f17), // 19719
(0x8, 0x300b), // 19716
(0x11, 0x3282), // 19720
(0x2, 0x33c8), // 19719
(0x26, 0x3402), // 19718
(0x1c, 0x3430), // 19718
(0x26, 0x34ca), // 19718
(0x16, 0x3579), // 19719
(0x1a, 0x3658), // 19717
(0xf, 0x3919), // 19719
(0x14, 0x39b2), // 19720
(0x2d, 0x3c31), // 19716
(0x17, 0x3d2d), // 19720
(0x36, 0x3d7f), // 19715
(0x35, 0x3e0f), // 19719
(0x3b, 0x3ee0), // 19719
(0x21, 0x3f47), // 19717
(0x8, 0x411d), // 19719
(0x30, 0x4292), // 19719
(0x31, 0x434b), // 19718
(0x25, 0x43f4), // 19718
(0xe, 0x456e), // 19719
(0x10, 0x4598), // 19720
(0x10, 0x47a6), // 19720
(0x21, 0x47e9), // 19719
(0x0, 0x488e), // 19716
(0x34, 0x48f4), // 19720
(0x22, 0x4902), // 19719
(0x15, 0x492c), // 19717
(0xc, 0x4b0f), // 19719
(0xf, 0x4dd0), // 19720
(0x6, 0x4e5c), // 19718
(0x27, 0x5091), // 19719
(0x2b, 0x5103), // 19719
(0x3b, 0x5110), // 19719
(0xa, 0x5113), // 19718
(0x26, 0x520e), // 19719
(0x6, 0x534b), // 19719
(0x0, 0x53aa), // 19719
(0x2d, 0x53b1), // 19719
(0x12, 0x5502), // 19720
(0x7, 0x5524), // 19718
(0x18, 0x56c4), // 19716
(0x7, 0x5a07), // 19719
(0x1b, 0x5bb1), // 19719
(0x28, 0x5c5b), // 19719
(0x2, 0x5c94), // 19719
(0x25, 0x5d34), // 19719
(0x35, 0x5e5c), // 19718
(0x2, 0x5f18), // 19719
(0x2c, 0x5fc8), // 19719
(0x32, 0x60b0), // 19719
(0x2d, 0x6115), // 19720
(0x14, 0x6168), // 19718
(0x2d, 0x6199), // 19718
(0x35, 0x6387), // 19719
(0x1f, 0x6388), // 19720
(0x30, 0x63c1), // 19719
(0x7, 0x64a3), // 19718
(0x2, 0x650c), // 19719
(0x2b, 0x6553), // 19720
(0x11, 0x6733), // 19719
(0x11, 0x6965), // 19719
(0x3f, 0x6a7c), // 19715
(0xa, 0x6add), // 19716
(0x19, 0x6b7d), // 19719
(0xf, 0x6ba2), // 19719
(0x2b, 0x6baa), // 19719
(0x29, 0x6bfe), // 19719
(0x1d, 0x6d72), // 19719
(0x37, 0x6e16), // 19718
(0x23, 0x705a), // 19720
(0x1f, 0x7088), // 19720
(0x21, 0x70cc), // 19720
(0x1f, 0x7195), // 19720
(0x1b, 0x71e4), // 19719
(0x15, 0x722d), // 19719
(0x1, 0x7236), // 19719
(0x18, 0x7754), // 19720
(0x28, 0x7c5d), // 19719
(0x13, 0x7eb7), // 19720
];


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemEq {
  // save state part 1
  position_vec: Vec3Fixed16,
  animation_state: AnimationState,
  animation_state_counter: u8,
  ski_jump_current_distance: u16,
  ski_flight_angle: i16,

  // save state part 2
  state: SkiJumpState,
  state_frame_count: u16,
  is_grounded: bool,
  raw_velocity_vec: Vec3Fixed16,
  ski_jump_upwards_movement_frames: u8,
  sub_state: SkiJumpSubState,

  // RNG values
  counter_529f6: u16,
  counter_529f8: u16,
}
fn mem_eq(mem: &Mem) -> MemEq {
  MemEq {
    position_vec: mem.position_vec,
    animation_state: mem.animation_state,
    animation_state_counter: mem.animation_state_counter,
    ski_jump_current_distance: mem.ski_jump_current_distance,
    ski_flight_angle: mem.ski_flight_angle,
    state: mem.state,
    state_frame_count: mem.state_frame_count,
    is_grounded: mem.is_grounded,
    raw_velocity_vec: mem.raw_velocity_vec,
    ski_jump_upwards_movement_frames: mem.ski_jump_upwards_movement_frames,
    sub_state: mem.sub_state,
    counter_529f6: mem.counter_529f6,
    counter_529f8: mem.counter_529f8,
  }
}



#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MemValue {
  total_velocity: i32,
  raw_velocity_vec_z: i16,
  position_vec_z: i16,
  ski_jump_current_distance: u16,
}
fn mem_value(mem: &Mem) -> MemValue {
  MemValue {
    ski_jump_current_distance: mem.ski_jump_current_distance,
    raw_velocity_vec_z: mem.raw_velocity_vec.z.raw(),
    position_vec_z: mem.position_vec.z.raw(),
    total_velocity: ((mem.raw_velocity_vec.x.raw() as i32) * (mem.raw_velocity_vec.x.raw() as i32)) + ((mem.raw_velocity_vec.y.raw() as i32) * (mem.raw_velocity_vec.y.raw() as i32)) + ((mem.raw_velocity_vec.z.raw() as i32) * (mem.raw_velocity_vec.z.raw() as i32)),
  }
}

fn next_inputs(mem: &Mem) -> Vec<InputFrame> {
  // {
  //   let mut res = Vec::new();
  //   for x in -3..=3 {
  //     res.push(InputFrame { x, y: 0, enter: false });
  //   }
  //   for y in -3..=3 {
  //     res.push(InputFrame { x: 0, y, enter: false });
  //   }
  //   res.push(InputFrame { x: 0, y: 0, enter: true });
  //   return res;
  // }
  if will_land_next_frame(mem) {
    vec![InputFrame { x: 0, y: 0, enter: true }]
  } else if let Some(_y) = flight_angle_correction(mem) {
    vec![InputFrame { x: 0, y: 0, enter: true }, InputFrame { x: 0, y: -3, enter: false }, InputFrame { x: 0, y: -2, enter: false }, InputFrame { x: 0, y: -1, enter: false }, InputFrame { x: 0, y: 0, enter: false }, InputFrame { x: 0, y: 1, enter: false }, InputFrame { x: 0, y: 2, enter: false }, InputFrame { x: 0, y: 3, enter: false }]
    // vec![InputFrame { x: 0, y, enter: false }]
  } else if mem.state == SkiJumpState::Landed {
    vec![InputFrame { x: -3, y: 0, enter: false }]
  } else {
    let liftoff = liftoff_input(mem);
    let xs = rolling_adjustment(mem);
    let mut result = Vec::new();
    for y in liftoff {
      for &x in &xs {
        result.push(InputFrame { x, y, enter: false });

      }
    }
    result
  }
}

#[allow(dead_code)]
fn optimize_config(mem_data: MemData) -> Option<(u16, Vec<InputFrame>)> {
  const MAX_CANDIDATES: usize = 100;

  let mut best_finished = None;

  let initial_mem = mem_data.to_mem();
  let initial_inputs = Vec::new();

  let mut current_candidates = vec![(initial_mem, initial_inputs)];

  while !current_candidates.is_empty() {
    let mut candidates = Vec::new();
    candidates.append(&mut current_candidates);

    for (mem, inputs) in candidates {
      'next_input: for next_input in next_inputs(&mem) {
        let mut next_mem = mem.clone();
        let mut next_inputs = inputs.clone();
        ski_jump_advance_frame_maybe_1f13e(&mut next_mem, next_input);
        next_inputs.push(next_input);
        if next_mem.animation_state == AnimationState::Landing {
          while !next_mem.has_failed && !next_mem.has_finished {
            ski_jump_advance_frame_maybe_1f13e(&mut next_mem, InputFrame { x: -3, y: 0, enter: false });
            next_inputs.push(InputFrame { x: -3, y: 0, enter: false });
          }
        }
        if next_mem.has_failed { continue 'next_input; }
        if next_mem.position_vec.z.raw() >= 0x2260 && !next_mem.is_grounded && next_mem.state == SkiJumpState::Jumping && next_mem.animation_state == AnimationState::Fly {
          if let Some(projected_distance) = simulate_flight(&next_mem) {  // discard flight trajectories which can't reach the needed distance
            // println!("{}", projected_distance);
            if projected_distance < 19735 { continue 'next_input; }
          }
        }
        if next_mem.has_finished {
          let value = mem_value(&next_mem);
          if let Some((best_value, _)) = &best_finished {
            if value <= *best_value { continue 'next_input; }
          }
          // deliberately skipping recording last input, since this is how real repplays work
          next_inputs.pop();
          best_finished = Some((value, next_inputs));
          continue 'next_input;
        }
        let next_eq = mem_eq(&next_mem);
        for (c_mem, _) in &current_candidates {
          if mem_eq(c_mem) == next_eq { continue 'next_input; }
        }

        current_candidates.push((next_mem, next_inputs));
        if current_candidates.len() > MAX_CANDIDATES {
          current_candidates.sort_by_key(|(mem, _)| mem_value(mem));
          current_candidates.reverse();
          while current_candidates.len() > MAX_CANDIDATES { current_candidates.pop(); }
        }
      }
    }
  }

  if let Some((val, inputs)) = best_finished {
    // println!("Found best state with distance {}", val.ski_jump_current_distance);
    Some((val.ski_jump_current_distance, inputs))
  } else {
    // println!("Failed to find any finishing states");
    None
  }
}

#[allow(dead_code)]
pub fn optimize() {
  let mut best_distance = 0;
  let mut best_mem_state = MemData::synthetic(0, 0, 0);
  let mut best_inputs = Vec::new();
  for &(counter_529f6, counter_529f8) in GOOD_RNG_VALUES {
    println!("{:x} {:x}", counter_529f6, counter_529f8);
  // for counter_529f8 in 0..=0x7fff {
  // (0..=0x7fff_u16).into_par_iter().for_each(|counter_529f8| {
    if counter_529f8 % 0x400 == 0 {
      println!("counter {:x}", counter_529f8);
    }
    // for counter_529f6 in 0..=0x3f {
    //   let angle = 0x2345;
    for angle in (0x1a00..=0x2700).step_by(10000) {
      let mem_state = MemData::synthetic(counter_529f6, counter_529f8, angle);
      if let Some((distance, inputs)) = optimize_config(mem_state) {
        if distance > best_distance {
          println!("found new best distance {} at counter {:x}:{:x}", distance, counter_529f6, counter_529f8);
          best_distance = distance;
          best_mem_state = mem_state;
          best_inputs = inputs;
        } else if distance >= 19730 {
          println!("good distance {} at counter {:x}:{:x}", distance, counter_529f6, counter_529f8);
        }
      }
    }
  }
  let player_state = PlayerData::synthetic();
  let replay_data = ReplayData {
    player_state,
    mem_state: best_mem_state,
    inputs: best_inputs,
  };
  replay_data.to_file("TEST2.RP5");
}





#[allow(dead_code)]
fn optimize_config_projected(mem_data: MemData) -> Option<(i16, Vec<InputFrame>)> {
  const MAX_CANDIDATES: usize = 100;

  let mut best_finished = None;

  let initial_mem = mem_data.to_mem();
  let initial_inputs = Vec::new();

  let mut current_candidates = vec![(initial_mem, initial_inputs)];

  // let mut cycles = 0;

  while !current_candidates.is_empty() {
    // println!("cycle {}", cycles);
    // cycles += 1;
    let mut candidates = Vec::new();
    candidates.append(&mut current_candidates);

    for (mem, inputs) in candidates {
      'next_input: for next_input in next_inputs(&mem) {
        let mut next_mem = mem.clone();
        ski_jump_advance_frame_maybe_1f13e(&mut next_mem, next_input);
        if next_mem.has_failed { continue 'next_input; }
        if let Some(projected_distance) = simulate_flight(&mem) {

        // }
        // if next_mem.has_finished {
          // let value = mem_value(&next_mem);
          if let Some((best_distance, _)) = &best_finished {
            if projected_distance <= *best_distance { continue 'next_input; }
          }
          // deliberately skipping recording last input, since this is how real replays work
          best_finished = Some((projected_distance, inputs.clone()))
        }
        let next_eq = mem_eq(&next_mem);
        for (c_mem, _) in &current_candidates {
          if mem_eq(c_mem) == next_eq { continue 'next_input; }
        }
        let mut next_inputs = inputs.clone();
        next_inputs.push(next_input);

        current_candidates.push((next_mem, next_inputs));
        if current_candidates.len() > MAX_CANDIDATES {
          current_candidates.sort_by_key(|(mem, _)| mem_value(mem));
          current_candidates.reverse();
          while current_candidates.len() > MAX_CANDIDATES { current_candidates.pop(); }
        }
      }
    }
  }

  if let Some((val, inputs)) = best_finished {
    // println!("Found best state with distance {}", val.ski_jump_current_distance);
    Some((val, inputs))
  } else {
    // println!("Failed to find any finishing states");
    None
  }
}
