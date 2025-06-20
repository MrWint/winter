use std::collections::HashSet;

struct RngState {
  counter_529f6: u16,
  counter_529f8: u16,
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
fn next_xor_rng_value(rng: &mut RngState) -> u16 {
  let bx = RNG_TABLE[rng.counter_529f6 as usize & 0x3f];
  rng.counter_529f6 = rng.counter_529f6.wrapping_add(1);
  rng.counter_529f8 = rng.counter_529f8.wrapping_add(0x1b65);
  (rng.counter_529f8 ^ bx) & 0x7fff
}
fn clamp_value(val: i16, min: i16, max: i16) -> i16 {
  if val < min { return min; }
  if val > max { return max; }
  val
}

#[allow(dead_code)]
pub fn enumerate_lr_random_reflections() {
  let mut all_deflections = HashSet::new();
  const NUM_FRAMES: usize = 80;
  for counter_529f6 in 0..=0x3f {
    for counter_529f8 in 0..=0x7fff {
      let mut rng = RngState { counter_529f6, counter_529f8, };
      let mut lr_random_deflection = 0;
      let mut deflections = Vec::with_capacity(NUM_FRAMES);
      for frame in 0..NUM_FRAMES {
        lr_random_deflection = clamp_value(lr_random_deflection + if frame % 2 == 0 { -1 } else { 1 } * (next_xor_rng_value(&mut rng) % 0x18) as i16, -0x28, 0x28);
        next_xor_rng_value(&mut rng);
        deflections.push(lr_random_deflection as i8);
      }
      all_deflections.insert(deflections);
    }
  }
  println!("possible variations: {}", all_deflections.len());
}
