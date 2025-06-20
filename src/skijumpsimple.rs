// Cleaned up version of the ski jump business logic, simplified to only the relevant parts.

use std::ops::{Add, Div, Mul, Neg, Sub};

const JMPTRACK_DATA: [JmpTrackSegment; 35] = [
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(      0), z: Fixed16(0x0000) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x02, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(      0), z: Fixed16(0x0280) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0x1c5), z: Fixed16(0x0500) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0x38a), z: Fixed16(0x0780) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0x54f), z: Fixed16(0x0A00) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0x714), z: Fixed16(0x0C80) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0x8d9), z: Fixed16(0x0F00) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0xa9e), z: Fixed16(0x1180) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0xc63), z: Fixed16(0x1400) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0xdf9), z: Fixed16(0x1680) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16( -0xf42), z: Fixed16(0x1900) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x1029), z: Fixed16(0x1B80) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x10c5), z: Fixed16(0x1E00) }, oc_next_segment_relative_z: Fixed16( 0x280), skip_over: false, segment_type: 0x00, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x1142), z: Fixed16(0x2080) }, oc_next_segment_relative_z: Fixed16( 0x140), skip_over: false, segment_type: 0x03, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x1181), z: Fixed16(0x21C0) }, oc_next_segment_relative_z: Fixed16(  0xA0), skip_over: false, segment_type: 0x03, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x11a1), z: Fixed16(0x2260) }, oc_next_segment_relative_z: Fixed16(-0x640), skip_over: true, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x17e1), z: Fixed16(0x2260) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x1b6a), z: Fixed16(0x2760) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x1ef3), z: Fixed16(0x2C60) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x227c), z: Fixed16(0x3160) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x25d8), z: Fixed16(0x3660) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x2904), z: Fixed16(0x3B60) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x2bff), z: Fixed16(0x4060) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x2ec7), z: Fixed16(0x4560) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x3159), z: Fixed16(0x4A60) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x33b5), z: Fixed16(0x4F60) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x3611), z: Fixed16(0x5460) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x386d), z: Fixed16(0x5960) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x3ac9), z: Fixed16(0x5E60) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x3d25), z: Fixed16(0x6360) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x3f81), z: Fixed16(0x6860) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x41dd), z: Fixed16(0x6D60) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x4439), z: Fixed16(0x7260) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x4695), z: Fixed16(0x7760) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
  JmpTrackSegment {position: Vec3Fixed16 { x: Fixed16(0), y: Fixed16(-0x48f1), z: Fixed16(0x7C60) }, oc_next_segment_relative_z: Fixed16( 0x500), skip_over: false, segment_type: 0x01, },
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed16(i16);
impl Fixed16 {
  fn zero() -> Fixed16 { Self(0) }
  pub fn from_raw_i16(v: i16) -> Self { Self(v) }
  pub fn raw(self) -> i16 {self.0 }
  
  fn abs(&self) -> Self {
    Self(self.0.abs())
  }
}
impl Add<Fixed16> for Fixed16 {
  type Output = Fixed16;
  fn add(self, rhs: Fixed16) -> Fixed16 {
    Fixed16(self.0 + rhs.0)
  }
}
impl Sub<Fixed16> for Fixed16 {
  type Output = Fixed16;
  fn sub(self, rhs: Fixed16) -> Fixed16 {
    Fixed16(self.0 - rhs.0)
  }
}
impl Mul<Fixed16> for Fixed16 {
  type Output = Fixed16;
  fn mul(self, rhs: Fixed16) -> Fixed16 {
    Fixed16(((self.0 as i32 * rhs.0 as i32) >> 15) as i16)
  }
}
impl Div<Fixed16> for Fixed16 {
  type Output = Fixed16;
  fn div(self, rhs: Fixed16) -> Fixed16 {
    Fixed16((((self.0 as i32) << 15) / rhs.0 as i32) as i16)
  }
}
impl Neg for Fixed16 {
  type Output = Fixed16;
  fn neg(self) -> Self::Output {
    Fixed16(-self.0)      
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Vec3Fixed16 {
  pub x: Fixed16,
  pub y: Fixed16,
  pub z: Fixed16,
}
impl Vec3Fixed16 {
  fn normalize(bx: Vec3Fixed16) -> (Vec3Fixed16, Fixed16) {  // returns normalized vec and previous length
    let mut ret = Vec3Fixed16::default();

    let mut si = Vec3Fixed16::length(bx);
    if si.0 == 0 {
      return (bx, si);
    }
    if si.0 < 0x7fff {
      si.0 += 1;
    }
    let var_2 = si.0 as i32;
    ret.x = Fixed16::from_raw_i16(((bx.x.0 as i32 * 0x7fff) / var_2) as i16);
    ret.y = Fixed16::from_raw_i16(((bx.y.0 as i32 * 0x7fff) / var_2) as i16);
    ret.z = Fixed16::from_raw_i16(((bx.z.0 as i32 * 0x7fff) / var_2) as i16);

    (ret, si)
  }
  fn length(bx: Vec3Fixed16) -> Fixed16 {
    let disi = bx.x.raw() as i32 * bx.x.raw() as i32 + bx.y.raw() as i32 * bx.y.raw() as i32 + bx.z.raw() as i32 * bx.z.raw() as i32;
    Fixed16::from_raw_i16(sqrt(disi as u32) as i16)
  }

  fn cross_product_scaled_2b8b8(ax: Vec3Fixed16, bx: Vec3Fixed16) -> Vec3Fixed16 { // returns bx x ax, scaled up to fit into a Fixed16
    let mut var_4 = (ax.y.raw() as i32 * bx.x.raw() as i32) - (ax.x.raw() as i32 * bx.y.raw() as i32);
    let mut var_c = (ax.z.raw() as i32 * bx.y.raw() as i32) - (ax.y.raw() as i32 * bx.z.raw() as i32);
    let mut var_8 = (ax.x.raw() as i32 * bx.z.raw() as i32) - (ax.z.raw() as i32 * bx.x.raw() as i32);
    // make sure length is < 1
    while var_4 > 0x49e6 || var_4 < -0x49e6 || var_8 > 0x49e6 || var_8 < -0x49e6 || var_c > 0x49e6 || var_c < -0x49e6 {
      var_4 >>= 1;
      var_8 >>= 1;
      var_c >>= 1;
    }
    Vec3Fixed16 {
      x: Fixed16::from_raw_i16(var_c as i16),
      y: Fixed16::from_raw_i16(var_8 as i16),
      z: Fixed16::from_raw_i16(var_4 as i16),
    }
  }
  fn cross_product(ax: Vec3Fixed16, bx: Vec3Fixed16) -> Vec3Fixed16 { // returns bx x ax
    Vec3Fixed16 {
      x: Fixed16::from_raw_i16((((ax.z.raw() as i32 * bx.y.raw() as i32) - (ax.y.raw() as i32 * bx.z.raw() as i32)) >> 15) as i16),
      y: Fixed16::from_raw_i16((((ax.x.raw() as i32 * bx.z.raw() as i32) - (ax.z.raw() as i32 * bx.x.raw() as i32)) >> 15) as i16),
      z: Fixed16::from_raw_i16((((ax.y.raw() as i32 * bx.x.raw() as i32) - (ax.x.raw() as i32 * bx.y.raw() as i32)) >> 15) as i16),
    }
  }
  fn dot_product(a: Vec3Fixed16, b: Vec3Fixed16) -> Fixed16 {
    a.x * b.x + a.y * b.y + a.z * b.z
  }
  fn dot_product_late_truncate(a: Vec3Fixed16, b: Vec3Fixed16) -> Fixed16 {
    Fixed16::from_raw_i16(((a.x.raw() as i32 * b.x.raw() as i32 + a.y.raw() as i32 * b.y.raw() as i32 + a.z.raw() as i32 * b.z.raw() as i32) >> 15) as i16)
  }
}
impl Add<Vec3Fixed16> for Vec3Fixed16 {
  type Output = Vec3Fixed16;
  fn add(self, rhs: Vec3Fixed16) -> Vec3Fixed16 {
    Vec3Fixed16 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}
impl Sub<Vec3Fixed16> for Vec3Fixed16 {
  type Output = Vec3Fixed16;
  fn sub(self, rhs: Vec3Fixed16) -> Vec3Fixed16 {
    Vec3Fixed16 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}
impl Mul<Fixed16> for Vec3Fixed16 {
  type Output = Vec3Fixed16;
  fn mul(self, rhs: Fixed16) -> Vec3Fixed16 {
    Vec3Fixed16 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}
impl Div<i16> for Vec3Fixed16 {
  type Output = Vec3Fixed16;
  fn div(self, rhs: i16) -> Vec3Fixed16 {
    Vec3Fixed16 {
      x: Fixed16::from_raw_i16(self.x.raw() / rhs),
      y: Fixed16::from_raw_i16(self.y.raw() / rhs),
      z: Fixed16::from_raw_i16(self.z.raw() / rhs),
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnimationState {
  #[default] Start = 2,
  Duck = 3,
  Straighten2 = 4,
  Fly = 5,
  Landing = 6,
  Landed = 7,
  Straighten = 9,
  ReDuck = 10,
  Braking = 11,
}
impl AnimationState {
  fn auto_advance_frames(self) -> u8 {
    match self {
      Self::Start => 10,
      Self::Duck => 1,
      Self::Straighten2 => 8,
      Self::Fly => 1,
      Self::Landing => 16,
      Self::Landed => 1,
      Self::Straighten => 4,
      Self::ReDuck => 4,
      Self::Braking => 0xff,
    }
  }
  fn transition(self, action: AnimationAction) -> AnimationState {  // from ski_jump_animation_action_map
    match self {
      Self::Start => match action {
        AnimationAction::Nothing => Self::Duck,
        AnimationAction::Duck => Self::Duck,
        _ => panic!("illegal animation state transition"),
      },
      Self::Duck => match action {
        AnimationAction::Nothing => Self::Duck,
        AnimationAction::Duck => Self::Duck,
        AnimationAction::Fly => Self::Straighten,
        AnimationAction::Braking => Self::Straighten,
        _ => panic!("illegal animation state transition"),
      },
      Self::Straighten2 => match action {
        AnimationAction::Nothing => Self::Fly,
        AnimationAction::Fly => Self::Fly,
        _ => panic!("illegal animation state transition"),
      },
      Self::Fly => match action {
        AnimationAction::Nothing => Self::Fly,
        AnimationAction::Fly => Self::Fly,
        AnimationAction::Landing => Self::Landing,
        _ => panic!("illegal animation state transition"),
      },
      Self::Landing | AnimationState::Landed => match action {
        AnimationAction::Nothing => Self::Landed,
        AnimationAction::Landing => Self::Landed,
        AnimationAction::Braking => Self::Braking,
        _ => panic!("illegal animation state transition"),
      },
      Self::Straighten => match action {
        AnimationAction::Nothing => Self::ReDuck,
        AnimationAction::Duck => Self::ReDuck,
        AnimationAction::Fly => Self::Straighten2,
        _ => panic!("illegal animation state transition"),
      },
      Self::ReDuck => match action {
        AnimationAction::Nothing => Self::Duck,
        AnimationAction::Duck => Self::Duck,
        AnimationAction::Fly => Self::Straighten,
        _ => panic!("illegal animation state transition"),
      },
      Self::Braking => match action {
        AnimationAction::Nothing => Self::Braking,
        AnimationAction::Duck => Self::Duck,
        AnimationAction::Fly => Self::Straighten,
        _ => panic!("illegal animation state transition"),
      },
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnimationAction {
  #[default] Nothing = 0,
  Duck = 3,
  Fly = 5,
  Landing = 6,
  Braking = 11,
}

#[derive(Clone, Debug, Default)]
pub struct JmpTrackSegment {
  position: Vec3Fixed16,
  oc_next_segment_relative_z: Fixed16,
  skip_over: bool,
  segment_type: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SkiJumpState {
  #[default] Jumping = 1,
  Landed = 2,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SkiJumpSubState {
  #[default] Unknown = 0,
  Rolling = 1,
  Flying = 2,
  Landed = 3,
}

#[derive(Clone, Debug, Default)]
pub struct Mem {
  // save state part 1
  pub position_vec: Vec3Fixed16,
  pub animation_state: AnimationState,
  pub animation_state_counter: u8,
  pub ski_jump_current_distance: u16,
  pub ski_flight_angle: i16,

  // save state part 2
  pub state: SkiJumpState,
  pub state_frame_count: u16,
  pub jmptrack_cur_segment: usize,
  pub surface_normal_vec: Vec3Fixed16,
  pub is_grounded: bool,
  pub animation_action: AnimationAction,
  pub raw_velocity_vec: Vec3Fixed16,
  pub normalized_velocity_vec: Vec3Fixed16,
  pub velocity_magnitude: u16,
  pub ski_jump_upwards_movement_frames: u8,
  pub ski_jump_upwards_movement_frames_calculated: bool,
  pub sub_state: SkiJumpSubState,
  pub ski_jump_never_lifted_off_flag: bool,
  pub landing_delay_counter: u8,
  pub lr_random_deflection: i16,

  // RNG values
  pub counter_529f6: u16,
  pub counter_529f8: u16,

  pub has_finished: bool,
  pub has_failed: bool,
}
impl Mem {
  pub fn compare_against_memdump(&self, file_name: &str) {
    let memdump = std::fs::read(file_name).expect("failed to read memdump");

    let counter_529f6 = u16::from_le_bytes(memdump[0x69f6..0x69f8].try_into().unwrap());
    let counter_529f8 = u16::from_le_bytes(memdump[0x69f8..0x69fa].try_into().unwrap());

    let position_x = i32::from_le_bytes(memdump[0x4406..0x440a].try_into().unwrap());
    let position_y = i32::from_le_bytes(memdump[0x440a..0x440e].try_into().unwrap());
    let position_z = i32::from_le_bytes(memdump[0x440e..0x4412].try_into().unwrap());
    let animation_state = memdump[0x4420];
    let animation_state_counter = memdump[0x4421];
    let ski_jump_current_distance = u16::from_le_bytes(memdump[0x4424..0x4426].try_into().unwrap());
    let ski_flight_angle = i16::from_le_bytes(memdump[0x44a6..0x44a8].try_into().unwrap());

    let state = memdump[0x4504];
    let state_frame_count = u16::from_le_bytes(memdump[0x4506..0x4508].try_into().unwrap());
    let jmptrack_cur_segment = u16::from_le_bytes(memdump[0x4508..0x450a].try_into().unwrap());
    let surface_normal_vec_x = i16::from_le_bytes(memdump[0x450c..0x450e].try_into().unwrap());
    let surface_normal_vec_y = i16::from_le_bytes(memdump[0x450e..0x4510].try_into().unwrap());
    let surface_normal_vec_z = i16::from_le_bytes(memdump[0x4510..0x4512].try_into().unwrap());
    let is_grounded = memdump[0x4518] != 0;
    let animation_action = memdump[0x451c];
    let raw_velocity_vec_x = i16::from_le_bytes(memdump[0x451e..0x4520].try_into().unwrap());
    let raw_velocity_vec_y = i16::from_le_bytes(memdump[0x4520..0x4522].try_into().unwrap());
    let raw_velocity_vec_z = i16::from_le_bytes(memdump[0x4522..0x4524].try_into().unwrap());
    let normalized_velocity_vec_x = i16::from_le_bytes(memdump[0x4524..0x4526].try_into().unwrap());
    let normalized_velocity_vec_y = i16::from_le_bytes(memdump[0x4526..0x4528].try_into().unwrap());
    let normalized_velocity_vec_z = i16::from_le_bytes(memdump[0x4528..0x452a].try_into().unwrap());
    let velocity_magnitude = u16::from_le_bytes(memdump[0x452a..0x452c].try_into().unwrap());
    let ski_jump_upwards_movement_frames = memdump[0x452e];
    let ski_jump_upwards_movement_frames_calculated = memdump[0x452f] != 0;
    let sub_state = memdump[0x4558];
    let ski_jump_never_lifted_off_flag = memdump[0x455a] != 0;
    let landing_delay_counter = memdump[0x455b];
    let lr_random_deflection = i16::from_le_bytes(memdump[0x455c..0x455e].try_into().unwrap());

    assert_eq!(self.counter_529f6, counter_529f6);
    assert_eq!(self.counter_529f8, counter_529f8);
    assert_eq!(self.position_vec.x.raw() as i32, position_x);
    assert_eq!(self.position_vec.y.raw() as i32, position_y);
    assert_eq!(self.position_vec.z.raw() as i32, position_z);
    assert_eq!(self.animation_state as u8, animation_state);
    assert_eq!(self.animation_state_counter, animation_state_counter);
    assert_eq!(self.ski_jump_current_distance, ski_jump_current_distance);
    assert_eq!(self.ski_flight_angle, ski_flight_angle);
    assert_eq!(self.state as u8, state);
    assert_eq!(self.state_frame_count, state_frame_count);
    assert_eq!(self.jmptrack_cur_segment as u16, jmptrack_cur_segment);
    assert_eq!(self.surface_normal_vec.x.raw(), surface_normal_vec_x);
    assert_eq!(self.surface_normal_vec.y.raw(), surface_normal_vec_y);
    assert_eq!(self.surface_normal_vec.z.raw(), surface_normal_vec_z);
    assert_eq!(self.is_grounded, is_grounded);
    assert_eq!(self.animation_action as u8, animation_action);
    assert_eq!(self.raw_velocity_vec.x.raw(), raw_velocity_vec_x);
    assert_eq!(self.raw_velocity_vec.y.raw(), raw_velocity_vec_y);
    assert_eq!(self.raw_velocity_vec.z.raw(), raw_velocity_vec_z);
    assert_eq!(self.normalized_velocity_vec.x.raw(), normalized_velocity_vec_x);
    assert_eq!(self.normalized_velocity_vec.y.raw(), normalized_velocity_vec_y);
    assert_eq!(self.normalized_velocity_vec.z.raw(), normalized_velocity_vec_z);
    assert_eq!(self.velocity_magnitude, velocity_magnitude);
    assert_eq!(self.ski_jump_upwards_movement_frames, ski_jump_upwards_movement_frames);
    assert_eq!(self.ski_jump_upwards_movement_frames_calculated, ski_jump_upwards_movement_frames_calculated);
    assert_eq!(self.sub_state as u8, sub_state);
    assert_eq!(self.ski_jump_never_lifted_off_flag, ski_jump_never_lifted_off_flag);
    assert_eq!(self.landing_delay_counter, landing_delay_counter);
    assert_eq!(self.lr_random_deflection, lr_random_deflection);
  }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct InputFrame {
  pub x: i8,
  pub y: i8,
  pub enter: bool,
}

#[allow(dead_code)]
pub fn ski_jump_advance_frame_maybe_1f13e(mem: &mut Mem, inputs: InputFrame) {
  assert!(!mem.has_failed);

  if mem.state_frame_count < 0x780 {
    mem.state_frame_count += 1;
  }
  ski_jump_simulate_1f7dc(mem, inputs);
  if mem.state == SkiJumpState::Landed && mem.state_frame_count > 0x50 {
    mem.has_finished = true;
  }
}

fn sqrt(v: u32) -> u16 {
  let mut ax = if v >= 0x10000 { 0x8000 } else { v as u16 >> 1 };
  if ax <= 1 { return ax; }
  loop {
    let bp = ax;
    ax = ((v / bp as u32) as u16 + bp + 1) / 2;
    if ax == bp {
      return ax;
    }
  }
}

fn ski_jump_simulate_1f7dc(mem: &mut Mem, inputs: InputFrame) {
  { // ski_jump_handle_inputs_1F902
    mem.animation_action = AnimationAction::Nothing;
    match mem.state {
      SkiJumpState::Jumping if mem.sub_state == SkiJumpSubState::Rolling => {
        if inputs.y > 0 { mem.animation_action = AnimationAction::Fly }
      },
      SkiJumpState::Jumping if mem.sub_state == SkiJumpSubState::Flying => {
        if mem.jmptrack_cur_segment < 18 && mem.animation_state != AnimationState::Fly {
          mem.animation_action = AnimationAction::Fly;
          if mem.animation_state == AnimationState::Duck {
            mem.ski_jump_never_lifted_off_flag = true;
          }
        }
        if mem.jmptrack_cur_segment > 18 && inputs.enter { mem.animation_action = AnimationAction::Landing }
      },
      SkiJumpState::Landed => {
        if inputs.x != 0 || mem.state_frame_count > 0x20 {
          mem.animation_action = AnimationAction::Braking;
        }
      },
      _ => {},
    }
  }
  { // ski_jump_handle_animation_state_transitions
    mem.animation_state_counter += 1;
    if mem.animation_state_counter >= mem.animation_state.auto_advance_frames() {
      if mem.animation_state == AnimationState::Straighten && mem.animation_action == AnimationAction::Fly && mem.jmptrack_cur_segment < 12 {
        mem.animation_state_counter -= 1;
      } else {
        mem.animation_state_counter = 0;
        mem.animation_state = mem.animation_state.transition(mem.animation_action);
        // unnecessary resetting of animation_action
        if mem.animation_action as u8 == mem.animation_state as u8 {
          mem.animation_action = AnimationAction::Nothing;
        }
      }
    }
  }
  let mut left_right_drift = Fixed16::zero();
  { // ski_jump_handle_animation_1FA42
    match mem.animation_state {
      AnimationState::Start => {},
      AnimationState::Duck | AnimationState::Straighten | AnimationState::ReDuck => {
        mem.lr_random_deflection = clamp_value(mem.lr_random_deflection + if mem.state_frame_count % 2 == 0 { -1 } else { 1 } * (next_xor_rng_value(mem) % 0x18) as i16, -0x28, 0x28);
        next_xor_rng_value(mem);
        left_right_drift = Fixed16::from_raw_i16(-0x18 * inputs.x as i16 + mem.lr_random_deflection);
      },
      AnimationState::Straighten2 => {
        if !mem.ski_jump_upwards_movement_frames_calculated {
          let var_2 = clamp_value((mem.position_vec.z.raw() as i16 - 0x22f0).abs() / 0x60, 0, 7) as u8;
          if !mem.ski_jump_never_lifted_off_flag {
            mem.ski_jump_upwards_movement_frames = 8 - var_2;
          }
          mem.ski_jump_upwards_movement_frames_calculated = true;
        }
      },
      AnimationState::Fly => {
        next_xor_rng_value(mem);
        mem.ski_flight_angle = clamp_value(mem.ski_flight_angle + ((inputs.y as i16 - 5) << 9) + (next_xor_rng_value(mem) as i16 % 0x1400), 0, 0x3fff);
      },
      AnimationState::Landing => {
        if mem.sub_state != SkiJumpSubState::Landed {
          if mem.animation_state_counter == 4 {
            mem.animation_state_counter = 0;
          }
          mem.landing_delay_counter += 1;
          if mem.landing_delay_counter > 9 {
            mem.has_failed = true;  // crashed
            return;
          }
        }
      },
      AnimationState::Landed => {
        left_right_drift = Fixed16::from_raw_i16(-0x28 * inputs.x as i16);
        next_xor_rng_value(mem);
      },
      AnimationState::Braking => {
        if mem.velocity_magnitude > 10 && mem.animation_state_counter >= 10 {
          mem.animation_state_counter = 4;
        }
      },
    }
  }
  { // ski_jump_update_physics_1FC84
    let gravity_vec = Vec3Fixed16 {
      x: Fixed16::zero(),
      y: Fixed16::from_raw_i16(if mem.ski_jump_upwards_movement_frames > 0 { 0x40 } else { -0x40 }),
      z: Fixed16::zero(),
    };
    let mut normal_force_vec = Vec3Fixed16::default();
    if mem.is_grounded {
      let si = mem.surface_normal_vec.y * -gravity_vec.y;  // negative dot product
      normal_force_vec = mem.surface_normal_vec * si;
    }
    let mut accelecration_vec = normal_force_vec + gravity_vec;
    // let mut vec_1c = Vec3Fixed16::default();
    if mem.is_grounded && mem.animation_state == AnimationState::Start {
      // vec_1c.z = Fixed16::from_raw_i16(0x80);
      accelecration_vec.z = accelecration_vec.z + Fixed16::from_raw_i16(0x80);  // initial push forward
    }
    if mem.is_grounded {
      let side_movement_vec = Vec3Fixed16::cross_product(mem.normalized_velocity_vec, mem.surface_normal_vec);
      let left_right_drift_vec = side_movement_vec * left_right_drift;
      accelecration_vec = accelecration_vec + left_right_drift_vec;
    }
    if mem.animation_state == AnimationState::Duck && ((mem.position_vec.x.raw() as i16).abs() >> 4) > 3 {
      mem.has_failed = true;  // too far off to the side, causes dust
      return;
    }
    let drag_vec = ski_jump_calculate_drag_vec(mem);
    accelecration_vec = accelecration_vec + drag_vec;

    mem.raw_velocity_vec = mem.raw_velocity_vec + accelecration_vec;
    if mem.state != SkiJumpState::Jumping && mem.velocity_magnitude < 6 {
      mem.raw_velocity_vec = Vec3Fixed16::default();
    }
    mem.position_vec = mem.position_vec + (mem.raw_velocity_vec / 16);

    // progress surface based on new position
    while (mem.position_vec.z - JMPTRACK_DATA[mem.jmptrack_cur_segment].position.z).raw() < -0x60 {
      if mem.jmptrack_cur_segment != 0 { mem.jmptrack_cur_segment -= 1; }
      if JMPTRACK_DATA[mem.jmptrack_cur_segment].skip_over { mem.jmptrack_cur_segment -= 1; }
    }
    while JMPTRACK_DATA[mem.jmptrack_cur_segment].oc_next_segment_relative_z <= (mem.position_vec.z - JMPTRACK_DATA[mem.jmptrack_cur_segment].position.z) {
      if mem.jmptrack_cur_segment < 0x21 { mem.jmptrack_cur_segment += 1; }
      if JMPTRACK_DATA[mem.jmptrack_cur_segment].skip_over { mem.jmptrack_cur_segment += 1; }
    }
    let (cur_track_segment_vertices_relative, next_track_segment_vertices_relative) = calculate_segment_vertices(mem.jmptrack_cur_segment);
    let segment_relative_position = mem.position_vec - JMPTRACK_DATA[mem.jmptrack_cur_segment].position;
    let segment_square = calculate_segment_square(cur_track_segment_vertices_relative, next_track_segment_vertices_relative, segment_relative_position);
    if segment_square != 1 {
      mem.has_failed = true;  // crashed, off track
      return;
    }
    mem.surface_normal_vec = calculate_surface_normal(segment_relative_position, segment_square, cur_track_segment_vertices_relative, next_track_segment_vertices_relative);

    mem.is_grounded = false;
    if mem.state == SkiJumpState::Jumping && mem.sub_state == SkiJumpSubState::Landed {
      ski_jump_update_state(mem, SkiJumpState::Landed);
    }
    { // ski_jump_check_grounded
      let diaxcx = mem.position_vec - (cur_track_segment_vertices_relative[segment_square] + JMPTRACK_DATA[mem.jmptrack_cur_segment].position);
      let di = -Vec3Fixed16::dot_product(mem.surface_normal_vec, diaxcx);

      if di.raw() > 2 {
        mem.is_grounded = true;
        mem.position_vec = mem.position_vec + (mem.surface_normal_vec * di);
        let si = Vec3Fixed16::dot_product_late_truncate(mem.surface_normal_vec, mem.raw_velocity_vec);

        mem.raw_velocity_vec = mem.raw_velocity_vec - (mem.surface_normal_vec * si);
      }
    }
    let (normalized_vec, length) = Vec3Fixed16::normalize(mem.raw_velocity_vec);
    mem.normalized_velocity_vec = normalized_vec;
    mem.velocity_magnitude = length.raw() as u16 / 16;
    if mem.ski_jump_upwards_movement_frames != 0 {
      mem.ski_jump_upwards_movement_frames -= 1;
    }
  }

  if mem.state == SkiJumpState::Jumping {
    if mem.sub_state == SkiJumpSubState::Unknown {
      if mem.jmptrack_cur_segment > 2 {
        mem.sub_state = SkiJumpSubState::Rolling;
      }
    } else if mem.sub_state == SkiJumpSubState::Rolling {
      if mem.jmptrack_cur_segment >= 14 {
        mem.sub_state = SkiJumpSubState::Flying;
        if mem.animation_state == AnimationState::Duck {
          mem.has_failed = true; // missed jump
          return;
        }
      }
    } else if mem.sub_state == SkiJumpSubState::Flying {
      if mem.position_vec.z.raw() >= 0x2260 {
        mem.ski_jump_current_distance = (((mem.position_vec.z.raw() as i32 - 0x2260) * 10) / 0x60) as u16;
      }
      if mem.is_grounded {
        if mem.animation_state == AnimationState::Landing {
          mem.sub_state = SkiJumpSubState::Landed;
        } else {
          mem.has_failed = true;  // crashed, didn't land
          return;
        }
      }
    }
  }
}

fn calculate_segment_vertices(jmptrack_cur_segment: usize) -> ([Vec3Fixed16; 4], [Vec3Fixed16; 4]) {
    let mut cur_track_segment_vertices_relative: [Vec3Fixed16; 4] = Default::default();
    let mut next_track_segment_vertices_relative: [Vec3Fixed16; 4] = Default::default();

    const SEGMENT_VERTEX_DATA: [[(Fixed16, Fixed16); 4]; 4] = [
      [(Fixed16(-0x1E0), Fixed16(    0)),   (Fixed16(-0x1A0), Fixed16(0)),   (Fixed16(0x1A0), Fixed16(0)),   (Fixed16( 0x1E0), Fixed16(    0))],  // ramp segment
      [(Fixed16(-0xC80), Fixed16(0x180)),   (Fixed16(-0x640), Fixed16(0)),   (Fixed16(0x640), Fixed16(0)),   (Fixed16( 0xC80), Fixed16(0x180))],  // ground segment
      [(Fixed16( -0xC0), Fixed16(    0)),   (Fixed16( -0x80), Fixed16(0)),   (Fixed16( 0x80), Fixed16(0)),   (Fixed16(  0xC0), Fixed16(    0))],  // start segment
      [(Fixed16(-0x280), Fixed16(    0)),   (Fixed16(-0x240), Fixed16(0)),   (Fixed16(0x240), Fixed16(0)),   (Fixed16( 0x280), Fixed16(    0))],  // ramp end segment
    ];

    let cur_segment_type = JMPTRACK_DATA[jmptrack_cur_segment].segment_type;
    for i in 0..4 {
      cur_track_segment_vertices_relative[i].x = -SEGMENT_VERTEX_DATA[cur_segment_type][i].0;
      cur_track_segment_vertices_relative[i].y = SEGMENT_VERTEX_DATA[cur_segment_type][i].1;
      cur_track_segment_vertices_relative[i].z = Fixed16::from_raw_i16(0);
    }
    let next_segment_relative_y = JMPTRACK_DATA[jmptrack_cur_segment + 1].position.y - JMPTRACK_DATA[jmptrack_cur_segment].position.y;
    let next_segment_type = JMPTRACK_DATA[jmptrack_cur_segment + 1].segment_type;
    for i in 0..4 {
      next_track_segment_vertices_relative[i].x = -SEGMENT_VERTEX_DATA[next_segment_type][i].0;
      next_track_segment_vertices_relative[i].y = SEGMENT_VERTEX_DATA[next_segment_type][i].1 + next_segment_relative_y;
      next_track_segment_vertices_relative[i].z = JMPTRACK_DATA[jmptrack_cur_segment].oc_next_segment_relative_z;
    }
    (cur_track_segment_vertices_relative, next_track_segment_vertices_relative)
}

fn calculate_segment_square(cur_track_segment_vertices_relative: [Vec3Fixed16; 4], next_track_segment_vertices_relative: [Vec3Fixed16; 4], segment_relative_position: Vec3Fixed16) -> usize {
  for i in 0..4 {
    let line_delta_z = next_track_segment_vertices_relative[i].z - cur_track_segment_vertices_relative[i].z;
    let line_delta_x = next_track_segment_vertices_relative[i].x - cur_track_segment_vertices_relative[i].x;
    if line_delta_x.abs() > line_delta_z.abs() {
      let dz_by_dx = line_delta_z / line_delta_x;
      let si = (segment_relative_position.x * dz_by_dx) + (cur_track_segment_vertices_relative[i].z - cur_track_segment_vertices_relative[i].x * dz_by_dx);
      if dz_by_dx >= Fixed16::zero() {
        if si >= segment_relative_position.z {
          return if i == 0 { 2 } else { i - 1 }; // to the right of line i (in square i-1..i)
        }
      } else {
        if si <= segment_relative_position.z {
          return if i == 0 { 2 } else { i - 1 }; // to the right of line i (in square i-1..i)
        }
      }
    } else if line_delta_x.abs() <= line_delta_z.abs() {
      let dx_by_dz = line_delta_x / line_delta_z;
      let line_x_at_position_z = (segment_relative_position.z * dx_by_dz) + (cur_track_segment_vertices_relative[i].x - cur_track_segment_vertices_relative[i].z * dx_by_dz);
      if segment_relative_position.x >= line_x_at_position_z {
        return if i == 0 { 2 } else { i - 1 }; // to the right of line i (in square i-1..i)
      }
    }
  }
  return 0;
}

fn calculate_surface_normal(segment_relative_position: Vec3Fixed16, segment_square: usize, cur_track_segment_vertices_relative: [Vec3Fixed16; 4], next_track_segment_vertices_relative: [Vec3Fixed16; 4]) -> Vec3Fixed16 {
  let var_1c = (segment_relative_position.x - cur_track_segment_vertices_relative[segment_square].x).raw() as i32 * (next_track_segment_vertices_relative[segment_square + 1].z - cur_track_segment_vertices_relative[segment_square].z).raw() as i32;
  let dxax = (segment_relative_position.z - cur_track_segment_vertices_relative[segment_square].z).raw() as i32 * (next_track_segment_vertices_relative[segment_square + 1].x - cur_track_segment_vertices_relative[segment_square].x).raw() as i32;

  let var_1a = if dxax > var_1c { // determine which triangle the player is on
    Vec3Fixed16::cross_product_scaled_2b8b8(next_track_segment_vertices_relative[segment_square + 1] - cur_track_segment_vertices_relative[segment_square], cur_track_segment_vertices_relative[segment_square + 1] - cur_track_segment_vertices_relative[segment_square])
  } else {
    Vec3Fixed16::cross_product_scaled_2b8b8(next_track_segment_vertices_relative[segment_square] - cur_track_segment_vertices_relative[segment_square], next_track_segment_vertices_relative[segment_square + 1] - cur_track_segment_vertices_relative[segment_square])
  };

  Vec3Fixed16::normalize(var_1a).0
}

fn ski_jump_calculate_drag_vec(mem: &Mem) -> Vec3Fixed16 {
  let mut speed = mem.velocity_magnitude;
  let di = match mem.animation_state {
    AnimationState::Duck => ((mem.position_vec.x.raw() as i16).abs() >> 4) + 10,
    AnimationState::Fly => {
      const SKI_JUMP_FLIGHT_ANGLE_BUCKETS: [i16; 8] = [0x600, 0xD00, 0x1300, 0x1A00, 0x2700, 0x2D00, 0x3400, 0x3A00];
      const SKI_JUMP_FLIGHT_ANGLE_DRAG_VALUES_MAYBE: [i16; 9] = [14, 12, 8, 6, 4, 6, 8, 12, 14];
      SKI_JUMP_FLIGHT_ANGLE_DRAG_VALUES_MAYBE[match SKI_JUMP_FLIGHT_ANGLE_BUCKETS.binary_search(&mem.ski_flight_angle) {
        Ok(p) => p,
        Err(p) => p,
      }]
    },
    AnimationState::Landing => 40,
    AnimationState::Landed => 96,
    AnimationState::Braking => {
      if speed < 40 { speed = 40; }
      350
    },
    AnimationState::Start | AnimationState::Straighten | AnimationState::Straighten2 | AnimationState::ReDuck => 18,
  };
  let mut drag_vec = mem.normalized_velocity_vec * Fixed16::from_raw_i16(((di as i32 * speed as i32) / -160) as i16);
  if mem.ski_jump_upwards_movement_frames != 0 {
    drag_vec.y = Fixed16::zero();
  }
  drag_vec
}

fn clamp_value(val: i16, min: i16, max: i16) -> i16 {
  if val < min { return min; }
  if val > max { return max; }
  val
}

fn ski_jump_update_state(mem: &mut Mem, ax: SkiJumpState) {
  if mem.state != ax {
    mem.state = ax;
    mem.state_frame_count = 0;
  }
}

const RNG_TABLE: [u16; 64] = [
  0xD7BF, 0xAD4A, 0x2089, 0x1EF6, 0x8EC6, 0xABF2, 0xFAB6, 0x49D7,
  0x1B68, 0x1F9C, 0x9920, 0xC7AD, 0xD678, 0x896D, 0xC363, 0x4E70,
  0x72EC, 0x6728, 0x54AA, 0x5E26, 0x3C22, 0x631C, 0x9B70, 0x18F0,
  0x9D69, 0xCA1F,  0x1BB, 0x53DC, 0xE630, 0x4099, 0xEBAC, 0xECF8,
  0x4D3B, 0xB298, 0xE278, 0x5FAA, 0x32D7, 0x8D2F, 0xF1D0, 0x94E6,
  0xA79C, 0x2E28, 0x5016, 0xFBD0, 0x3098, 0x9909, 0x3F51, 0xDBEF,
  0xC761, 0x6D7E, 0x3433, 0xE7E9, 0x2426, 0x1804, 0x3839, 0x1406,
  0x3B01, 0x4725, 0x8DF1, 0xA62F,  0x5A6, 0xA07E, 0x95DE, 0x8FF7,
];
fn next_xor_rng_value(mem: &mut Mem) -> u16 {
  let bx = RNG_TABLE[mem.counter_529f6 as usize & 0x3f];
  mem.counter_529f6 = mem.counter_529f6.wrapping_add(1);
  mem.counter_529f8 = mem.counter_529f8.wrapping_add(0x1b65);
  (mem.counter_529f8 ^ bx) & 0x7fff
}













const SUFRACE_NORMALS_16_PLUS: [Vec3Fixed16; 18] = [
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(26754), z: Fixed16(18915) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(26754), z: Fixed16(18915) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(26754), z: Fixed16(18915) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(27196), z: Fixed16(18272) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(27667), z: Fixed16(17551) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(28144), z: Fixed16(16775) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(28633), z: Fixed16(15927) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29140), z: Fixed16(14980) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
  Vec3Fixed16 { x: Fixed16(0), y: Fixed16(29631), z: Fixed16(13982) },
];

#[derive(Clone, Debug)]
struct FlightMem {
  position_vec: Vec3Fixed16,
  raw_velocity_vec: Vec3Fixed16,
}
impl FlightMem {
  fn new(mut position_vec: Vec3Fixed16, raw_velocity_vec: Vec3Fixed16) -> FlightMem {
    position_vec.x = Fixed16::zero();
    FlightMem { position_vec, raw_velocity_vec }
  }
}

// simplified simulation of a single frame of airborne motion.
fn simulate_flight_frame(mem: &mut FlightMem, with_upwards_movement: bool, drag: u16) -> bool {
  assert!(mem.position_vec.z.raw() >= 0x2260);

  let (normalized_velocity_vec, length) = Vec3Fixed16::normalize(mem.raw_velocity_vec);
  let velocity_magnitude = length.raw() as u16 / 16;
  
  let gravity_vec = Vec3Fixed16 {
    x: Fixed16::zero(),
    y: Fixed16::from_raw_i16(if with_upwards_movement { 0x40 } else { -0x40 }),
    z: Fixed16::zero(),
  };
  let mut accelecration_vec = gravity_vec;
  let mut drag_vec = normalized_velocity_vec * Fixed16::from_raw_i16(((drag as i32 * velocity_magnitude as i32) / -160) as i16);
  if with_upwards_movement {
    drag_vec.y = Fixed16::zero();
  }
  accelecration_vec = accelecration_vec + drag_vec;

  mem.raw_velocity_vec = mem.raw_velocity_vec + accelecration_vec;
  mem.position_vec = mem.position_vec + (mem.raw_velocity_vec / 16);

  let mut jmptrack_cur_segment = 16;
  while JMPTRACK_DATA[jmptrack_cur_segment].oc_next_segment_relative_z + JMPTRACK_DATA[jmptrack_cur_segment].position.z <= mem.position_vec.z { jmptrack_cur_segment += 1; }
  let surface_normal_vec = SUFRACE_NORMALS_16_PLUS[jmptrack_cur_segment - 16];

  let mut is_grounded = false;
  { // ski_jump_check_grounded
    let segment_relative_position = mem.position_vec - JMPTRACK_DATA[jmptrack_cur_segment].position;
    let di = -Vec3Fixed16::dot_product(surface_normal_vec, segment_relative_position);

    if di.raw() > 2 {
      is_grounded = true;
      mem.position_vec = mem.position_vec + (surface_normal_vec * di);
      let si = Vec3Fixed16::dot_product_late_truncate(surface_normal_vec, mem.raw_velocity_vec);

      mem.raw_velocity_vec = mem.raw_velocity_vec - (surface_normal_vec * si);
    }
  }
  mem.position_vec.x = Fixed16::zero();

  is_grounded
}


// pre-calculates the surface normals of all track surfaces.
#[allow(dead_code)]
pub fn calc_surface_normals() {
  let segment_square = 1;
  for jmptrack_cur_segment in 0..JMPTRACK_DATA.len()-1 {
    if JMPTRACK_DATA[jmptrack_cur_segment].skip_over { continue; }
    let (cur_track_segment_vertices_relative, next_track_segment_vertices_relative) = calculate_segment_vertices(jmptrack_cur_segment);

    let surface_normal_1 = Vec3Fixed16::normalize(Vec3Fixed16::cross_product_scaled_2b8b8(next_track_segment_vertices_relative[segment_square + 1] - cur_track_segment_vertices_relative[segment_square], cur_track_segment_vertices_relative[segment_square + 1] - cur_track_segment_vertices_relative[segment_square])).0;
    let surface_normal_2 = Vec3Fixed16::normalize(Vec3Fixed16::cross_product_scaled_2b8b8(next_track_segment_vertices_relative[segment_square] - cur_track_segment_vertices_relative[segment_square], next_track_segment_vertices_relative[segment_square + 1] - cur_track_segment_vertices_relative[segment_square])).0;

    println!("{jmptrack_cur_segment} 1: {surface_normal_1:?}  2: {surface_normal_2:?}");
  }
}

// heuristic simulating flight and estimating the maximum achievable distance
pub fn simulate_flight(mem: &Mem) -> Option<i16> {
  if mem.position_vec.z.raw() < 0x2260 { return None; }

  assert!(mem.jmptrack_cur_segment >= 16); // impossible based on position
  assert!(!mem.is_grounded); // impossible based on position
  assert_eq!(mem.state, SkiJumpState::Jumping);
  assert_eq!(mem.sub_state, SkiJumpSubState::Flying);
  assert!(mem.animation_state != AnimationState::Duck);  // impossible, would have missed jump

  // possible animation states: AnimationState::Straighten, AnimationState::Straighten2, AnimationState::Fly

  let mut animation_state = mem.animation_state;
  let mut animation_state_counter = mem.animation_state_counter;
  let mut ski_jump_upwards_movement_frames = mem.ski_jump_upwards_movement_frames;

  let mut flight_mem = FlightMem::new(mem.position_vec, mem.raw_velocity_vec);

  if animation_state == AnimationState::Straighten {
    while animation_state_counter + 1 < AnimationState::Straighten.auto_advance_frames() {
      if simulate_flight_frame(&mut flight_mem, false, 18) { return Some(-1); };
      animation_state_counter += 1;
    }
    ski_jump_upwards_movement_frames = 7 - clamp_value((flight_mem.position_vec.z.raw() as i16 - 0x22f0).abs() / 0x60, 0, 7) as u8;
    if simulate_flight_frame(&mut flight_mem, true, 18) { return Some(-1); };
    animation_state = AnimationState::Straighten2;
    animation_state_counter = 0;
  }
  if animation_state == AnimationState::Straighten2 {
    assert!(animation_state_counter + ski_jump_upwards_movement_frames < AnimationState::Straighten2.auto_advance_frames());
    while animation_state_counter + 1 < AnimationState::Straighten2.auto_advance_frames() {
      if simulate_flight_frame(&mut flight_mem, ski_jump_upwards_movement_frames > 0, 18) { return Some(-1); };
      animation_state_counter += 1;
      ski_jump_upwards_movement_frames = ski_jump_upwards_movement_frames.saturating_sub(1);
    }
    if simulate_flight_frame(&mut flight_mem, false, 4) { return Some(-1); };
    animation_state = AnimationState::Fly;
  }
  assert!(animation_state == AnimationState::Fly);

  let mut fly_states = Vec::new();
  fly_states.push(flight_mem.clone());
  while !simulate_flight_frame(&mut flight_mem, false, 4) {
    fly_states.push(flight_mem.clone());
  }
  let mut first_delay_index = 0;
  for i in (0..fly_states.len()).rev() {
    let expected_frames = fly_states.len() - i;
    let mut num_landing_frames = 0;
    let mut landing_mem = fly_states[i].clone();
    while !simulate_flight_frame(&mut landing_mem, false, if expected_frames - num_landing_frames <= 8 { 40 } else { 14 } ) {
      num_landing_frames += 1;
      assert!(num_landing_frames <= expected_frames);
    }
    num_landing_frames += 1;
    if num_landing_frames > expected_frames {
      first_delay_index = i;
      break;
    }
  }
  // println!("{}", fly_states.len() - first_delay_index);
  let chosen_index = first_delay_index.saturating_sub(3);
  let landing_mem = &fly_states[chosen_index];
  let mut best_distance = -1;
  let expected_frames = fly_states.len() - chosen_index;
  for initial_drag_0 in [4, 6, 8, 12, 14] {
    for initial_drag_1 in [4, 6, 8, 12, 14] {
      for initial_drag_2 in [4, 6, 8, 12, 14] {
        for initial_drag_3 in [4, 6, 8, 12, 14] {
          for initial_drag_4 in [6, 8, 12, 14] {
            let mut num_landing_frames = 0;
            let mut landing_mem = landing_mem.clone();
            while !simulate_flight_frame(&mut landing_mem, false,
                if expected_frames - num_landing_frames <= 8 { 40 } else
                if num_landing_frames == 0 { initial_drag_0 } else {
                  if num_landing_frames == 1 { initial_drag_1 } else {
                    if num_landing_frames == 2 { initial_drag_2 } else { if num_landing_frames == 3 { initial_drag_3 } else { if num_landing_frames == 4 { initial_drag_4 } else { 14 } } } } }) {
              num_landing_frames += 1;
              assert!(num_landing_frames <= expected_frames);
            }
            if best_distance < landing_mem.position_vec.z.raw() {
              best_distance = landing_mem.position_vec.z.raw();
            }
          }
        }
      }
    }
  }
  Some(best_distance)
}








// determines sensible left/right inputs based on the current state
pub fn rolling_adjustment(mem: &Mem) -> Vec<i8> {
  if mem.state != SkiJumpState::Jumping { return vec![0]; }
  if !mem.is_grounded { return vec![0]; }
  if mem.animation_state != AnimationState::Duck && (mem.animation_state != AnimationState::Straighten || mem.animation_state_counter + 1 >= AnimationState::Straighten.auto_advance_frames()) { return vec![0]; }

  let next_xor_rng_value = (mem.counter_529f8.wrapping_add(0x1b65) ^ RNG_TABLE[mem.counter_529f6 as usize & 0x3f]) & 0x7fff;
  let lr_random_deflection = clamp_value(mem.lr_random_deflection + if mem.state_frame_count % 2 == 0 { -1 } else { 1 } * (next_xor_rng_value % 0x18) as i16, -0x28, 0x28);

  let drag_vec = {
    let di = match mem.animation_state {
      AnimationState::Duck => ((mem.position_vec.x.raw() as i16).abs() >> 4) + 10,
      _ => 18,
    };
    mem.normalized_velocity_vec * Fixed16::from_raw_i16(((di as i32 * mem.velocity_magnitude as i32) / -160) as i16)
  };

  let gravity_vec = Vec3Fixed16 {
    x: Fixed16::zero(),
    y: Fixed16::from_raw_i16(if mem.ski_jump_upwards_movement_frames > 0 { 0x40 } else { -0x40 }),
    z: Fixed16::zero(),
  };
  let normal_force_vec = mem.surface_normal_vec * (mem.surface_normal_vec.y * -gravity_vec.y);

  // max swerving keeping centered
  {
    let side_movement_vec = Vec3Fixed16::cross_product(mem.normalized_velocity_vec, mem.surface_normal_vec);

    let maxright_left_right_drift = Fixed16::from_raw_i16(-0x48 + lr_random_deflection);
    let maxright_left_right_drift_vec = side_movement_vec * maxright_left_right_drift;
    let maxright_accelecration_vec = normal_force_vec + gravity_vec + drag_vec + maxright_left_right_drift_vec;
    let maxright_raw_velocity_vec = mem.raw_velocity_vec + maxright_accelecration_vec;

    let maxleft_left_right_drift = Fixed16::from_raw_i16(0x48 + lr_random_deflection);
    let maxleft_left_right_drift_vec = side_movement_vec * maxleft_left_right_drift;
    let maxleft_accelecration_vec = normal_force_vec + gravity_vec + drag_vec + maxleft_left_right_drift_vec;
    let maxleft_raw_velocity_vec = mem.raw_velocity_vec + maxleft_accelecration_vec;

    assert!(maxleft_raw_velocity_vec.x >= maxright_raw_velocity_vec.x);

    if maxleft_raw_velocity_vec.x.raw() < 0x0 { return vec![-3] } // full left to straighten out
    if maxright_raw_velocity_vec.x.raw() > 0x0 { return vec![3] } // full right to straighten out

    return vec![-3, 0, 3];
  }
}

// determines when it is the right time to lift off
pub fn liftoff_input(mem: &Mem) -> Vec<i8> {
  if mem.animation_state == AnimationState::Straighten { // continue lifting off
    return vec![3];
  }
  if mem.animation_state == AnimationState::Duck {
    let position_z = mem.position_vec.z.raw() as i16;
    let in_4_frames = position_z + 4 * (mem.raw_velocity_vec.z.raw() / 16);
    let in_5_frames = position_z + 5 * (mem.raw_velocity_vec.z.raw() / 16);
  
    if in_4_frames <= 0x22f0 - 0x60 { // not far enough, wait
      return vec![0];
    }
    if in_4_frames >= 0x22f0 + 0x60 { // too far, no time to jump anymore
      return vec![];
    }
    if in_5_frames >= 0x22f0 + 0x60 { // now or never, else we miss our chance
      return vec![3];
    }
    return vec![0, 3];  // we can jump now or wait another frame
  }
  return vec![0];  // nothing to do
}

// determines which angle adjustment holds the ski angle centered
pub fn flight_angle_correction(mem: &Mem) -> Option<i8> {
  if mem.state != SkiJumpState::Jumping { return None; }
  if (mem.animation_state != AnimationState::Straighten2 || mem.animation_state_counter + 1 < AnimationState::Straighten2.auto_advance_frames()) && mem.animation_state != AnimationState::Fly { return None; }

  let next_xor_rng_value = (mem.counter_529f8.wrapping_add(0x1b65 + 0x1b65) ^ RNG_TABLE[mem.counter_529f6.wrapping_add(1) as usize & 0x3f]) & 0x7fff;
  
  let mut best_input = 0;
  let mut best_ski_flight_deviation = 0x7fff;
  for y in [-3, -2, -1, 0, 1, 2, 3] {
    let ski_flight_angle = clamp_value(mem.ski_flight_angle + ((y as i16 - 5) << 9) + (next_xor_rng_value as i16 % 0x1400), 0, 0x3fff);
    let deviation = (0x2080 - ski_flight_angle).abs();
    if deviation < best_ski_flight_deviation {
      best_input = y;
      best_ski_flight_deviation = deviation;
    }
  }

  Some(best_input)
}

// detemines whether the player will land on the next frame
pub fn will_land_next_frame(mem: &Mem) -> bool {
  if mem.state != SkiJumpState::Jumping { return false; }
  if mem.sub_state != SkiJumpSubState::Flying { return false; }
  if mem.jmptrack_cur_segment <= 18 { return false; }
  if mem.animation_state != AnimationState::Fly { return false; }
  if mem.is_grounded { return false; }
  if mem.ski_jump_upwards_movement_frames > 0 { return false; }

  let gravity_vec = Vec3Fixed16 {
    x: Fixed16::zero(),
    y: Fixed16::from_raw_i16(-0x40),
    z: Fixed16::zero(),
  };
  let drag_vec = mem.normalized_velocity_vec * Fixed16::from_raw_i16(((40 * mem.velocity_magnitude as i32) / -160) as i16);
  let accelecration_vec = gravity_vec + drag_vec;
  let raw_velocity_vec = mem.raw_velocity_vec + accelecration_vec;
  let position_vec = mem.position_vec + (raw_velocity_vec / 16);
  let mut jmptrack_cur_segment = mem.jmptrack_cur_segment;

  // progress surface based on new position
  if (position_vec.z - JMPTRACK_DATA[jmptrack_cur_segment].position.z).raw() < -0x60 {
    return false;
  }
  while JMPTRACK_DATA[jmptrack_cur_segment].oc_next_segment_relative_z <= (position_vec.z - JMPTRACK_DATA[jmptrack_cur_segment].position.z) {
    if jmptrack_cur_segment >= 0x21 { return false; }
    jmptrack_cur_segment += 1;
  }
  let (cur_track_segment_vertices_relative, next_track_segment_vertices_relative) = calculate_segment_vertices(jmptrack_cur_segment);
  let segment_relative_position = position_vec - JMPTRACK_DATA[jmptrack_cur_segment].position;
  let segment_square = calculate_segment_square(cur_track_segment_vertices_relative, next_track_segment_vertices_relative, segment_relative_position);
  if segment_square != 1 {
    return false;
  }
  let surface_normal_vec = calculate_surface_normal(segment_relative_position, segment_square, cur_track_segment_vertices_relative, next_track_segment_vertices_relative);

  let diaxcx = position_vec - (cur_track_segment_vertices_relative[segment_square] + JMPTRACK_DATA[jmptrack_cur_segment].position);
  let di = -Vec3Fixed16::dot_product(surface_normal_vec, diaxcx);

  di.raw() > 2
}
