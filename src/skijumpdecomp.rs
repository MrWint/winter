// Direct reconstruction of the business logic of the ski jump event, decompiled from the game files

use std::{num::Wrapping, ops::{Add, Div, Mul, Neg, Sub}};

// Data extracted from the JMPTRACK.BIN resource, containing vertex data for the segments of the ramp and ground.
const JMPTRACK_DATA: [JmpTrackSegment; 35] = [
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(      0), z: Fixed32(0x0000) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0x0000), o_10: 0x00, o11_segment_type: 0x02, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(      0), z: Fixed32(0x0280) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0x1c5), z: Fixed32(0x0500) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0x38a), z: Fixed32(0x0780) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0x54f), z: Fixed32(0x0A00) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0x714), z: Fixed32(0x0C80) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0x8d9), z: Fixed32(0x0F00) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0xa9e), z: Fixed32(0x1180) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0xc63), z: Fixed32(0x1400) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xE400), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0xdf9), z: Fixed32(0x1680) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xEA00), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32( -0xf42), z: Fixed32(0x1900) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xF100), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x1029), z: Fixed32(0x1B80) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xF600), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x10c5), z: Fixed32(0x1E00) }, oc_next_segment_relative_z: Fixed16( 0x280), o_e: Wrapping(0xF800), o_10: 0x00, o11_segment_type: 0x00, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x1142), z: Fixed32(0x2080) }, oc_next_segment_relative_z: Fixed16( 0x140), o_e: Wrapping(0xF800), o_10: 0x00, o11_segment_type: 0x03, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x1181), z: Fixed32(0x21C0) }, oc_next_segment_relative_z: Fixed16(  0xA0), o_e: Wrapping(0xF800), o_10: 0x00, o11_segment_type: 0x03, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x11a1), z: Fixed32(0x2260) }, oc_next_segment_relative_z: Fixed16(-0x640), o_e: Wrapping(0x0000), o_10: 0x08, o11_segment_type: 0x01, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x17e1), z: Fixed32(0x2260) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0xF8, 0xF8, 0xFF, 0xFF, 0x1F, 0xE8, 0xFF, 0xFF, 0x60, 0x22, 0x00, 0x00, 0x01, 0x00, 0x44, 0x07, 0x00, 0x00, 0x1F, 0xE8, 0xFF, 0xFF, 0x60, 0x22, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x1b6a), z: Fixed32(0x2760) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x68, 0xF7, 0xFF, 0xFF, 0x96, 0xE4, 0xFF, 0xFF, 0x60, 0x27, 0x00, 0x00, 0x01, 0x00, 0x44, 0x07, 0x00, 0x00, 0x96, 0xE4, 0xFF, 0xFF, 0x60, 0x27, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x1ef3), z: Fixed32(0x2C60) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xE000), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x80, 0xF8, 0xFF, 0xFF, 0x0D, 0xE1, 0xFF, 0xFF, 0x60, 0x2C, 0x00, 0x00, 0x01, 0x00, 0x84, 0x08, 0x00, 0x00, 0x0D, 0xE1, 0xFF, 0xFF, 0x60, 0x2C, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x227c), z: Fixed32(0x3160) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xE200), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x84, 0xF9, 0xFF, 0xFF, 0x84, 0xDD, 0xFF, 0xFF, 0x60, 0x31, 0x00, 0x00, 0x01, 0x00, 0x98, 0x08, 0x00, 0x00, 0x84, 0xDD, 0xFF, 0xFF, 0x60, 0x31, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x25d8), z: Fixed32(0x3660) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xE400), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0xA4, 0xF7, 0xFF, 0xFF, 0x28, 0xDA, 0xFF, 0xFF, 0x60, 0x36, 0x00, 0x00, 0x01, 0x00, 0x20, 0x08, 0x00, 0x00, 0x28, 0xDA, 0xFF, 0xFF, 0x60, 0x36, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x2904), z: Fixed32(0x3B60) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xE600), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0xC0, 0xF9, 0xFF, 0xFF, 0xFC, 0xD6, 0xFF, 0xFF, 0x60, 0x3B, 0x00, 0x00, 0x01, 0x00, 0x80, 0x07, 0x00, 0x00, 0xFC, 0xD6, 0xFF, 0xFF, 0x60, 0x3B, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x2bff), z: Fixed32(0x4060) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xE800), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0xAC, 0xF9, 0xFF, 0xFF, 0x01, 0xD4, 0xFF, 0xFF, 0x60, 0x40, 0x00, 0x00, 0x01, 0x00, 0x1C, 0x07, 0x00, 0x00, 0x01, 0xD4, 0xFF, 0xFF, 0x60, 0x40, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x2ec7), z: Fixed32(0x4560) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEA00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x08, 0xF8, 0xFF, 0xFF, 0x39, 0xD1, 0xFF, 0xFF, 0x60, 0x45, 0x00, 0x00, 0x01, 0x00, 0x58, 0x07, 0x00, 0x00, 0x39, 0xD1, 0xFF, 0xFF, 0x60, 0x45, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x3159), z: Fixed32(0x4A60) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0xB8, 0xF7, 0xFF, 0xFF, 0xA7, 0xCE, 0xFF, 0xFF, 0x60, 0x4A, 0x00, 0x00, 0x01, 0x00, 0x40, 0x06, 0x00, 0x00, 0xA7, 0xCE, 0xFF, 0xFF, 0x60, 0x4A, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x33b5), z: Fixed32(0x4F60) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x84, 0xF9, 0xFF, 0xFF, 0x4B, 0xCC, 0xFF, 0xFF, 0x60, 0x4F, 0x00, 0x00, 0x01, 0x00, 0x48, 0x08, 0x00, 0x00, 0x4B, 0xCC, 0xFF, 0xFF, 0x60, 0x4F, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x3611), z: Fixed32(0x5460) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x54, 0xF7, 0xFF, 0xFF, 0xEF, 0xC9, 0xFF, 0xFF, 0x60, 0x54, 0x00, 0x00, 0x01, 0x00, 0xBC, 0x07, 0x00, 0x00, 0xEF, 0xC9, 0xFF, 0xFF, 0x60, 0x54, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x386d), z: Fixed32(0x5960) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x70, 0xF9, 0xFF, 0xFF, 0x93, 0xC7, 0xFF, 0xFF, 0x60, 0x59, 0x00, 0x00, 0x01, 0x00, 0x54, 0x06, 0x00, 0x00, 0x93, 0xC7, 0xFF, 0xFF, 0x60, 0x59, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x3ac9), z: Fixed32(0x5E60) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x44, 0xF8, 0xFF, 0xFF, 0x37, 0xC5, 0xFF, 0xFF, 0x60, 0x5E, 0x00, 0x00, 0x01, 0x00, 0xF8, 0x07, 0x00, 0x00, 0x37, 0xC5, 0xFF, 0xFF, 0x60, 0x5E, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x3d25), z: Fixed32(0x6360) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x70, 0xF9, 0xFF, 0xFF, 0xDB, 0xC2, 0xFF, 0xFF, 0x60, 0x63, 0x00, 0x00, 0x01, 0x00, 0x48, 0x08, 0x00, 0x00, 0xDB, 0xC2, 0xFF, 0xFF, 0x60, 0x63, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x3f81), z: Fixed32(0x6860) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0xA8, 0xF8, 0xFF, 0xFF, 0x7F, 0xC0, 0xFF, 0xFF, 0x60, 0x68, 0x00, 0x00, 0x01, 0x00, 0x5C, 0x08, 0x00, 0x00, 0x7F, 0xC0, 0xFF, 0xFF, 0x60, 0x68, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x41dd), z: Fixed32(0x6D60) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0x80, 0xF8, 0xFF, 0xFF, 0x23, 0xBE, 0xFF, 0xFF, 0x60, 0x6D, 0x00, 0x00, 0x01, 0x00, 0x94, 0x07, 0x00, 0x00, 0x23, 0xBE, 0xFF, 0xFF, 0x60, 0x6D, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x4439), z: Fixed32(0x7260) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0xA4, 0xF7, 0xFF, 0xFF, 0xC7, 0xBB, 0xFF, 0xFF, 0x60, 0x72, 0x00, 0x00, 0x01, 0x00, 0x34, 0x08, 0x00, 0x00, 0xC7, 0xBB, 0xFF, 0xFF, 0x60, 0x72, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x4695), z: Fixed32(0x7760) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x01, 0x00, 0xF8, 0xF8, 0xFF, 0xFF, 0x6B, 0xB9, 0xFF, 0xFF, 0x60, 0x77, 0x00, 0x00, 0x01, 0x00, 0x5C, 0x08, 0x00, 0x00, 0x6B, 0xB9, 0xFF, 0xFF, 0x60, 0x77, 0x00, 0x00], },
  JmpTrackSegment {o_0: Vec3Fixed32 { x: Fixed32(0), y: Fixed32(-0x48f1), z: Fixed32(0x7C60) }, oc_next_segment_relative_z: Fixed16( 0x500), o_e: Wrapping(0xEC00), o_10: 0x00, o11_segment_type: 0x01, _rest: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], },
];

// 32-bit fixed point numbers as used in the game.
#[derive(Clone, Copy, Debug, Default)]
struct Fixed32(i32);
impl Fixed32 {
  fn from_raw_i32(v: i32) -> Self { Self(v) }
  fn to_fixed16(self) -> Fixed16 { Fixed16(self.0 as i16) }
  fn raw(self) -> i32 {self.0 }
}
impl Add<Fixed32> for Fixed32 {
  type Output = Fixed32;
  fn add(self, rhs: Fixed32) -> Fixed32 {
    Fixed32(self.0 + rhs.0)
  }
}
impl Sub<Fixed32> for Fixed32 {
  type Output = Fixed32;
  fn sub(self, rhs: Fixed32) -> Fixed32 {
    Fixed32(self.0 - rhs.0)
  }
}

// 16-bit fixed point numbers as used in the game.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Fixed16(i16);
impl Fixed16 {
  fn zero() -> Fixed16 { Self(0) }
  fn from_raw_i16(v: i16) -> Self { Self(v) }
  fn to_fixed32(self) -> Fixed32 { Fixed32(self.0 as i32) }
  fn raw(self) -> i16 {self.0 }
  
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

// 3d vectors of 16-bit fixed-point numbers
#[derive(Clone, Copy, Debug, Default)]
struct Vec3Fixed16 {
  x: Fixed16,
  y: Fixed16,
  z: Fixed16,
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
  fn to_fixed32(self) -> Vec3Fixed32 {
    Vec3Fixed32 {
      x: self.x.to_fixed32(),
      y: self.y.to_fixed32(),
      z: self.z.to_fixed32(),
    }
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

#[derive(Clone, Copy, Debug, Default)]
struct Vec3Fixed32 {
  x: Fixed32,
  y: Fixed32,
  z: Fixed32,
}
impl Vec3Fixed32 {
  fn to_fixed16(self) -> Vec3Fixed16 {
    Vec3Fixed16 {
      x: self.x.to_fixed16(),
      y: self.y.to_fixed16(),
      z: self.z.to_fixed16(),
    }
  }
}
impl Add<Vec3Fixed32> for Vec3Fixed32 {
  type Output = Vec3Fixed32;
  fn add(self, rhs: Vec3Fixed32) -> Vec3Fixed32 {
    Vec3Fixed32 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum AnimationState {
  #[default] Unknown = 0,
  NotStated = 1,
  Start = 2,
  Duck = 3,
  Straighten2 = 4,
  Fly = 5,
  Landing = 6,
  Landed = 7,
  Crashed = 8,
  Straighten = 9,
  ReDuck = 10,
  Braking = 11,
}
impl AnimationState {
  fn auto_advance_frames(self) -> u8 {
    match self {
      Self::Unknown => 1,
      Self::NotStated => 1,
      Self::Start => 10,
      Self::Duck => 1,
      Self::Straighten2 => 8,
      Self::Fly => 1,
      Self::Landing => 16,
      Self::Landed => 1,
      Self::Crashed => 0xff,
      Self::Straighten => 4,
      Self::ReDuck => 4,
      Self::Braking => 0xff,
    }
  }
  fn can_override(self) -> bool {
    match self {
      Self::Unknown => true,
      Self::NotStated => true,
      Self::Fly => true,
      _ => false,
    }
  }
  fn transition(self, action: AnimationAction) -> AnimationState {  // from ski_jump_animation_action_map
    match self {
      Self::Unknown => Self::Unknown,
      Self::NotStated => match action {
        AnimationAction::Nothing => Self::NotStated,
        AnimationAction::Duck => Self::Start,
        _ => Self::Unknown,
      },
      Self::Start => match action {
        AnimationAction::Nothing => Self::Duck,
        AnimationAction::Duck => Self::Duck,
        _ => Self::Unknown,
      },
      Self::Duck => match action {
        AnimationAction::Nothing => Self::Duck,
        AnimationAction::Duck => Self::Duck,
        AnimationAction::Fly => Self::Straighten,
        AnimationAction::Braking => Self::Straighten,
        _ => Self::Unknown,
      },
      Self::Straighten2 => match action {
        AnimationAction::Nothing => Self::Fly,
        AnimationAction::Fly => Self::Fly,
        _ => Self::Unknown,
      },
      Self::Fly => match action {
        AnimationAction::Nothing => Self::Fly,
        AnimationAction::Fly => Self::Fly,
        AnimationAction::Landing => Self::Landing,
        _ => Self::Unknown,
      },
      Self::Landing | AnimationState::Landed => match action {
        AnimationAction::Nothing => Self::Landed,
        AnimationAction::Landing => Self::Landed,
        AnimationAction::Braking => Self::Braking,
        _ => Self::Unknown,
      },
      Self::Crashed => Self::Crashed,
      Self::Straighten => match action {
        AnimationAction::Nothing => Self::ReDuck,
        AnimationAction::Duck => Self::ReDuck,
        AnimationAction::Fly => Self::Straighten2,
        _ => Self::Unknown,
      },
      Self::ReDuck => match action {
        AnimationAction::Nothing => Self::Duck,
        AnimationAction::Duck => Self::Duck,
        AnimationAction::Fly => Self::Straighten,
        _ => Self::Unknown,
      },
      Self::Braking => match action {
        AnimationAction::Nothing => Self::Braking,
        AnimationAction::Duck => Self::Duck,
        AnimationAction::Fly => Self::Straighten,
        _ => Self::Unknown,
      },
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum AnimationAction {
  #[default] Nothing = 0,
  Duck = 3,
  Fly = 5,
  Landing = 6,
  Braking = 11,
}

#[derive(Clone, Debug, Default)]
struct JmpTrackSegment {
  o_0: Vec3Fixed32,
  oc_next_segment_relative_z: Fixed16,
  o_e: Wrapping<u16>,
  o_10: u8,
  o11_segment_type: u8,
  _rest: [u8; 28],
}

#[derive(Debug, Default, PartialEq, Eq)]
enum SkiJumpState {
  #[default] NotStarted = 0,
  Jumping = 1,
  Landed = 2,
  Crashed = 3,
  Exiting = 4,
  _InMenu = 5,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum SkiJumpSubState {
  #[default] Unknown = 0,
  Rolling = 1,
  Flying = 2,
  Landed = 3,
}

#[derive(Debug, Default)]
struct DustParticleData {
  o0: u8,
  o1: u8,
  o2: Vec3Fixed32,
}

#[derive(Debug, Default)]
struct Mem {
  // save state part 1
  word_503f8: u16,
  dword_503fa: Vec3Fixed32,
  position_vec: Vec3Fixed32,
  // word_50412: u16,
  word_50414: Vec3Fixed16,
  // word_5041a: Vec3Fixed16,
  animation_state: AnimationState,
  animation_state_counter: u8,
  animation_randomness_maybe_50422: u8,
  ski_jump_current_distance: u16,
  dust_particle_data: [DustParticleData; 8],
  displayed_velocity: u16,
  // vec_50498: Vec3Fixed32,
  // byte_504a4: u8,  // write only
  ski_jump_ending_turn: u8,
  ski_flight_angle: i16,
  // ski_jump_crash_ski_data_maybe_504a8: [u32; 76/4],

  // save state part 2
  state: SkiJumpState,
  state_frame_count: u16,
  jmptrack_cur_segment: u16,
  // word_5050a: u16,  // unused write only
  surface_normal_vec: Vec3Fixed16,
  is_grounded: bool,
  // word_5051a: u16,
  animation_action: AnimationAction,
  // byte_5051d: u8,  // always 0?
  raw_velocity_vec: Vec3Fixed16,
  normalized_velocity_vec: Vec3Fixed16,
  velocity_magnitude: u16,
  segment_square: u16,
  ski_jump_upwards_movement_frames: u8,
  ski_jump_upwards_movement_frames_calculated: bool,
  unk_dust_maybe_50530: u8,
  ski_jump_unk_dust_maybe_50531: u8,
  unk_dust_maybe_50532: u8,
  unk_dust_maybe_50533: u8,
  word_50534: bool,
  word_50548: Vec3Fixed16,
  word_5054e: Vec3Fixed16,
  sub_state: SkiJumpSubState,
  sub_state_frame_count: u8,
  ski_jump_never_lifted_off_flag: bool,
  landing_delay_counter: u8,
  lr_random_deflection: i16,

  // not part of save state
  left_right_input: i16,
  up_down_input: i16,
  enter_pressed: bool,
  left_right_drift: Fixed16,
  drag_vec: Vec3Fixed16,
  next_jmptrack_segment: u16,
  jmptrack_segment_buffer: JmpTrackSegment,
  jmptrack_next_segment_ptr: usize,
  // ski_jump_unk_x_diff_divide_z_diff_505a0: Fixed16, // always 0
  jmptrack_next_segment_relative_z: Fixed16,
  segment_relative_position: Vec3Fixed16,
  cur_track_segment_vertices_relative: [Vec3Fixed16; 4],
  next_track_segment_vertices_relative: [Vec3Fixed16; 4],
  grounded_position: Vec3Fixed32,

  // RNG values
  counter_529f6: u16,
  counter_529f8: u16,
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

#[allow(dead_code)]
fn ski_jump_advance_frame_maybe_1f13e(mem: &mut Mem) {
  // various guard clause checks
  mem.left_right_input = 0;
  mem.up_down_input = 0;
  mem.enter_pressed = false;
  mem.left_right_drift = Fixed16::zero();

  if mem.state_frame_count < 0x780 {
    mem.state_frame_count += 1;
  }
  match mem.state {
    SkiJumpState::NotStarted => {
      // may move directly to exit if in replay
      ski_jump_read_inputs_1f414(mem);
      ski_jump_simulate_1f7dc(mem);
      // ski_jump_handle_crash_ski_data_20C78(mem);
      ski_jump_camera_control_20688(mem);
      mem.displayed_velocity = 0;
    },
    SkiJumpState::Jumping => {
      if 0x22 > mem.jmptrack_cur_segment {
        if JMPTRACK_DATA[mem.jmptrack_cur_segment as usize].o_10 & 4 == 0 {  // always true
          mem.word_503f8 = sub_2ad18(mem.word_503f8, 1);
        }
      }
      ski_jump_read_inputs_1f414(mem);
      ski_jump_simulate_1f7dc(mem);
      // ski_jump_handle_crash_ski_data_20C78(mem);
      ski_jump_camera_control_20688(mem);
    },
    SkiJumpState::Landed => {
      ski_jump_read_inputs_1f414(mem);
      ski_jump_simulate_1f7dc(mem);
      // ski_jump_handle_crash_ski_data_20C78(mem);
      ski_jump_camera_control_20688(mem);
      // sub_2AE1A()
      // setting word_52FC2
      if mem.state_frame_count > 0x50 {
        // record ski_jump_current_distance
        ski_jump_update_state(mem, SkiJumpState::Exiting);
      }
      if mem.state_frame_count > 0x10 {
        mem.displayed_velocity = mem.velocity_magnitude;
        if mem.displayed_velocity < 10 {
          mem.displayed_velocity = 0;
        }
      }
    },
    SkiJumpState::Crashed => {
      todo!()
    },
    SkiJumpState::Exiting => {
      todo!()
    },
    SkiJumpState::_InMenu => {
      todo!()
    },
  }

  todo!()
}

fn sub_2ad18(bx_frame_counter: u16, ax_increment: u16) -> u16 {
  const MAX_FRAME_COUNT: u16 = 10000;  // based on current discipline

  std::cmp::min(MAX_FRAME_COUNT, bx_frame_counter + ax_increment)
}

fn ski_jump_simulate_1f7dc(mem: &mut Mem) {
  'ski_jump_handle_inputs_1F902: { // ski_jump_handle_inputs_1F902
    mem.animation_action = AnimationAction::Nothing;
    match mem.state {
      SkiJumpState::NotStarted => {
        if mem.up_down_input < 0 { mem.animation_action = AnimationAction::Duck }
      },
      SkiJumpState::Jumping if mem.sub_state == SkiJumpSubState::Rolling => {
        if mem.up_down_input > 0 { mem.animation_action = AnimationAction::Fly }
      },
      SkiJumpState::Jumping if mem.sub_state == SkiJumpSubState::Flying => {
        if mem.jmptrack_cur_segment < 18 && mem.animation_state != AnimationState::Fly {
          mem.animation_action = AnimationAction::Fly;
          if mem.animation_state == AnimationState::Duck {
            mem.ski_jump_never_lifted_off_flag = true;
          }
        }
        if mem.jmptrack_cur_segment > 18 && mem.enter_pressed { mem.animation_action = AnimationAction::Landing }
      },
      SkiJumpState::Landed => {
        if mem.animation_state == AnimationState::Braking { break 'ski_jump_handle_inputs_1F902; }
        if mem.left_right_input != 0 {
          mem.animation_action = AnimationAction::Braking;
          if mem.left_right_input < 0 {
            mem.ski_jump_ending_turn = 0;
          } else {
            mem.ski_jump_ending_turn = 1;
          }
        } else if mem.state_frame_count > 0x20 {
          mem.animation_action = AnimationAction::Braking;
          mem.ski_jump_ending_turn = next_xor_rng_value(mem) as u8 & 1
        }
      },
      _ => break 'ski_jump_handle_inputs_1F902,
    }
  }
  'ski_jump_handle_animation_state_transitions: { // ski_jump_handle_animation_state_transitions
    mem.animation_state_counter += 1;
    if mem.animation_state_counter < mem.animation_state.auto_advance_frames()
      && (mem.animation_action == AnimationAction::Nothing
        || mem.animation_action as u8 == mem.animation_state as u8
        || !mem.animation_state.can_override()) {
      break 'ski_jump_handle_animation_state_transitions;
    }
    if mem.animation_state == AnimationState::Straighten && mem.animation_action == AnimationAction::Fly && mem.jmptrack_cur_segment < 12 {
      mem.animation_state_counter -= 1;
      break 'ski_jump_handle_animation_state_transitions;
    }
    mem.animation_state_counter = 0;
    mem.animation_state = mem.animation_state.transition(mem.animation_action);
    // unnecessary resetting of animation_action
    if mem.animation_action as u8 == mem.animation_state as u8 {
      mem.animation_action = AnimationAction::Nothing;
    }
  }
  { // ski_jump_handle_animation_1FA42
    match mem.animation_state {
      AnimationState::Start => {
        if mem.state == SkiJumpState::NotStarted {
          ski_jump_update_state(mem, SkiJumpState::Jumping);
        }
        // execute sub_2E73C if ski_jump_animation_state_counter_50421 == 8
      },
      AnimationState::Duck | AnimationState::Straighten | AnimationState::ReDuck => {
        mem.lr_random_deflection = clamp_value(mem.lr_random_deflection + if mem.state_frame_count % 2 == 0 { -1 } else { 1 } * (next_xor_rng_value(mem) % 0x18) as i16, -0x28, 0x28);
        mem.animation_randomness_maybe_50422 = next_xor_rng_value(mem) as u8;
        mem.left_right_drift = Fixed16::from_raw_i16(-0x18 * mem.left_right_input + mem.lr_random_deflection);
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
        mem.animation_randomness_maybe_50422 = next_xor_rng_value(mem) as u8;
        mem.ski_flight_angle = clamp_value(mem.ski_flight_angle + ((mem.up_down_input - 5) << 9) + (next_xor_rng_value(mem) as i16 % 0x1400), 0, 0x3fff);
      },
      AnimationState::Landing => {
        if mem.sub_state != SkiJumpSubState::Landed {
          if mem.animation_state_counter == 4 {
            mem.animation_state_counter = 0;
          }
          mem.landing_delay_counter += 1;
          if mem.landing_delay_counter > 9 {
            ski_jump_crash_20086(mem);
          }
      }
      },
      AnimationState::Landed => {
        mem.left_right_drift = Fixed16::from_raw_i16(-0x28 * mem.left_right_input);
        mem.animation_randomness_maybe_50422 = next_xor_rng_value(mem) as u8;
      },
      AnimationState::Crashed => {
        todo!()
      },
      AnimationState::Braking => {
        if mem.velocity_magnitude > 10 && mem.animation_state_counter >= 10 {
          mem.animation_state_counter = 4;
        }
        if mem.velocity_magnitude > 100 {
          mem.unk_dust_maybe_50532 = 2;
          mem.unk_dust_maybe_50533 = 0;
          mem.unk_dust_maybe_50530 = 4;
        } else if mem.velocity_magnitude > 40 {
          mem.unk_dust_maybe_50532 = 1;
          mem.unk_dust_maybe_50533 = 0;
          mem.unk_dust_maybe_50530 = 4;
        }
      },
      _ => {},
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
      let left_right_drift_vec = side_movement_vec * mem.left_right_drift;
      accelecration_vec = accelecration_vec + left_right_drift_vec;
    }
    ski_jump_calculate_drag_vec(mem);
    accelecration_vec = accelecration_vec + mem.drag_vec;
    if mem.animation_state == AnimationState::NotStated {
      accelecration_vec = Vec3Fixed16::default();
    }

    mem.raw_velocity_vec = mem.raw_velocity_vec + accelecration_vec;
    if mem.state != SkiJumpState::NotStarted && mem.state != SkiJumpState::Jumping && mem.velocity_magnitude < 6 {
      mem.raw_velocity_vec = Vec3Fixed16::default();
    }
    mem.position_vec = mem.position_vec + (mem.raw_velocity_vec / 16).to_fixed32();

    if mem.animation_state == AnimationState::Crashed && mem.position_vec.z.raw() < 0x22f0 {
      mem.position_vec.x = Fixed32::from_raw_i32(clamp_dword(mem.position_vec.x.raw(), -0x1c7, 0x1c7));
    }

    // progress surface based on new position
    let mut found_surface = false;
    for _si in 0..2 {
      if found_surface { break; }
      if mem.jmptrack_cur_segment + 1 >= 0x22 { break; }
      load_next_geometry_segment(mem, mem.jmptrack_cur_segment);
      mem.segment_square = calc_player_segment_square(mem, mem.position_vec);
      if mem.segment_square == 0x82 {  // off to the side, limit to inbounds
        mem.segment_square = 2;
      } else if mem.segment_square == 0x83 {
        mem.segment_square = 0;
      }
      if mem.segment_square == 0x80 { // in front, load earlier segment
        loop {
          if mem.jmptrack_cur_segment != 0 {
            mem.jmptrack_cur_segment -= 1;
            load_next_geometry_segment(mem, mem.jmptrack_cur_segment);
          }
          if mem.jmptrack_segment_buffer.o_10 & 8 == 0 { break; }
        }
      } else if mem.segment_square == 0x81 { // behind, load next segment
        loop {
          if 0x21 > mem.jmptrack_cur_segment {
            mem.jmptrack_cur_segment += 1;
            load_next_geometry_segment(mem, mem.jmptrack_cur_segment);
          }
          if mem.jmptrack_segment_buffer.o_10 & 8 == 0 { break; }
        }
      } else if mem.segment_square < 3 {
        load_next_geometry_segment(mem, mem.jmptrack_cur_segment);
        /* mem.word_5050a = */calculate_surface_normal(mem, mem.segment_square);
        found_surface = true;
      }
    }

    mem.is_grounded = false;
    // mem.word_5051a += 1;
    if found_surface {
      if mem.state == SkiJumpState::Jumping && mem.sub_state == SkiJumpSubState::Landed {
        ski_jump_update_state(mem, SkiJumpState::Landed);
      }
      let (new_pos, new_velocity, grounded) = ski_jump_check_grounded(mem, mem.position_vec, Some(mem.raw_velocity_vec), mem.segment_square);
      mem.position_vec = new_pos;
      mem.raw_velocity_vec = new_velocity.unwrap();
      mem.is_grounded = grounded;

      // position adjusted forward by ground penetration (attempt for sub-frame adjustments?)
      // mem.vec_50498 = mem.grounded_position;
      // mem.vec_50498.z = mem.vec_50498.z + Fixed32::from_raw_i32(((mem.position_vec.y - mem.vec_50498.y).raw() * 3) / 2);

      // if mem.is_grounded {
      //   mem.word_5051a = 0;
      // }
      // sub_203fe(mem);
    }
    // loc_20014
    if (mem.state == SkiJumpState::Jumping || mem.state == SkiJumpState::Landed) && mem.animation_state != AnimationState::Crashed && (mem.segment_square == 0 || mem.segment_square == 2) {
      if mem.is_grounded || mem.state_frame_count < 0x30 {
        ski_jump_crash_20086(mem);
      }
    } else {
      // if mem.is_grounded && mem.byte_5051d != 0 {  // can never happen
      //   ski_jump_crash_20086(mem);
      // }
    }
    let (normalized_vec, length) = Vec3Fixed16::normalize(mem.raw_velocity_vec);
    mem.normalized_velocity_vec = normalized_vec;
    mem.velocity_magnitude = length.raw() as u16 / 16;
    if mem.ski_jump_upwards_movement_frames != 0 {
      mem.ski_jump_upwards_movement_frames -= 1;
    }
  }

  if mem.state == SkiJumpState::Jumping {
    if mem.sub_state_frame_count < 200 {
      mem.sub_state_frame_count += 1;
    }
    if mem.sub_state == SkiJumpSubState::Unknown {
      if mem.jmptrack_cur_segment > 2 {
        mem.sub_state = SkiJumpSubState::Rolling;
        mem.sub_state_frame_count = 0;
      }
      if mem.state_frame_count > 8 {
        mem.displayed_velocity = mem.velocity_magnitude;
      }
    } else if mem.sub_state == SkiJumpSubState::Rolling {
      mem.displayed_velocity = mem.velocity_magnitude;
      if mem.jmptrack_cur_segment >= 14 {
        // sub_2e73c(0xb);
        mem.sub_state = SkiJumpSubState::Flying;
        mem.sub_state_frame_count = 0;
      }
    } else if mem.sub_state == SkiJumpSubState::Flying {
      if mem.sub_state_frame_count > 0x10 {
        mem.displayed_velocity = mem.velocity_magnitude;
      }
      // if mem.raw_velocity_vec.y.raw() < -0x400 && mem.jmptrack_cur_segment > 18 {
      //   mem.byte_504a4 = 1;
      // }
      if mem.position_vec.z.raw() >= 0x2260 {
        mem.ski_jump_current_distance = (((mem.position_vec.z.raw() - 0x2260) * 10) / 0x60) as u16;
      }
      if mem.is_grounded {
        mem.unk_dust_maybe_50532 = 2;
        mem.unk_dust_maybe_50533 = 0;
        mem.unk_dust_maybe_50530 = 8;
        if mem.animation_state == AnimationState::Landing {
          mem.sub_state = SkiJumpSubState::Landed;
          mem.sub_state_frame_count = 0;
          // sub_2e73c(0x17);
        } else {
          ski_jump_crash_20086(mem);
        }
      }
    } else if mem.sub_state == SkiJumpSubState::Landed {
      if mem.sub_state_frame_count > 0x10 {
        mem.displayed_velocity = mem.velocity_magnitude;
      }
    }
  }
  ski_jump_create_dust(mem);
}

fn ski_jump_create_dust(mem: &mut Mem) {
  let mut si = None;
  let mut di = None;
  for i in 0..8 {
    if mem.dust_particle_data[i].o0 != 0 {
      mem.dust_particle_data[i].o0 -= 1;
    } else if si.is_none() {
      si = Some(i);
    } else if di.is_none() {
      di = Some(i);
    }
  }
  if mem.unk_dust_maybe_50530 == 0 { return; }
  mem.unk_dust_maybe_50530 -= 1;
  if !mem.is_grounded { return; }
  if mem.ski_jump_unk_dust_maybe_50531 != 0 {
    mem.ski_jump_unk_dust_maybe_50531 -= 1;
    return;
  }
  if si.is_none() || di.is_none() { return; }
  let si = si.unwrap();
  let di = di.unwrap();

  let var_2 = if mem.animation_state == AnimationState::Crashed { 0 } else { -0x20 };
  mem.dust_particle_data[si].o0 = 8;
  mem.dust_particle_data[si].o1 = mem.unk_dust_maybe_50532;
  mem.dust_particle_data[si].o2.z = mem.position_vec.z;
  mem.dust_particle_data[si].o2.x = mem.position_vec.x + Fixed32::from_raw_i32(((next_xor_rng_value(mem) % 0x20) * 4) as i32 - 0x40);
  mem.dust_particle_data[si].o2.y = mem.position_vec.y + Fixed32::from_raw_i32(var_2);

  mem.dust_particle_data[di].o0 = 8;
  mem.dust_particle_data[di].o1 = mem.unk_dust_maybe_50532;
  mem.dust_particle_data[di].o2.z = mem.position_vec.z;
  mem.dust_particle_data[di].o2.x = mem.position_vec.x;
  mem.dust_particle_data[di].o2.y = mem.position_vec.y + Fixed32::from_raw_i32(var_2);

  mem.ski_jump_unk_dust_maybe_50531 = mem.unk_dust_maybe_50533;

  if mem.state == SkiJumpState::Jumping && mem.sub_state == SkiJumpSubState::Rolling {
    // sub_2e73c(0xa);
  }
}

// fn sub_203fe(mem: &mut Mem) {
//   if mem.segment_square < 3 {
//     mem.word_5041a = Vec3Fixed16 {
//       x: Fixed16::from_raw_i16((mem.position_vec.x.raw() / 0x20) as i16),
//       y: Fixed16::from_raw_i16((mem.position_vec.y.raw() / 0x20) as i16),
//       z: Fixed16::from_raw_i16((mem.position_vec.z.raw() / 0x20) as i16),
//     };
//   }
// }

fn ski_jump_check_grounded(mem: &mut Mem, bx_position_vec: Vec3Fixed32, ax_velocity_vec: Option<Vec3Fixed16>, dx_segment_square: u16) -> (Vec3Fixed32, Option<Vec3Fixed16>, bool) {

  let mut new_position = bx_position_vec;
  let mut new_velocity= None;
  let mut grounded = false;

  let diaxcx = bx_position_vec.to_fixed16() - (mem.cur_track_segment_vertices_relative[dx_segment_square as usize] + mem.jmptrack_segment_buffer.o_0.to_fixed16());
  let di = -Vec3Fixed16::dot_product(mem.surface_normal_vec, diaxcx);

  mem.grounded_position = bx_position_vec + (mem.surface_normal_vec * di).to_fixed32();

  if di.raw() > 2 {
    grounded = true;
    new_position = mem.grounded_position;
    if let Some(ax_velocity_vec) = ax_velocity_vec {
      let si = Vec3Fixed16::dot_product_late_truncate(mem.surface_normal_vec, ax_velocity_vec);

      new_velocity = Some(ax_velocity_vec - (mem.surface_normal_vec * si));
    }
  }

  (new_position, new_velocity, grounded)
}

fn ski_jump_calculate_drag_vec(mem: &mut Mem) {
  let mut speed = mem.velocity_magnitude;
  let di = match mem.animation_state {
    AnimationState::Duck => {
      let di = ((mem.position_vec.x.raw() as i16).abs() >> 4) + 10;
      if di > 17 {
        mem.unk_dust_maybe_50532 = 1;
        mem.unk_dust_maybe_50533 = 1;
        mem.unk_dust_maybe_50530 = 4;
      } else if di > 13 && mem.unk_dust_maybe_50530 == 0 {
        mem.unk_dust_maybe_50532 = 0;
        mem.unk_dust_maybe_50533 = 2;
        mem.unk_dust_maybe_50530 = 4;
      }
      di
    },
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
    AnimationState::Crashed => if mem.is_grounded { 280 } else { 8 },
    AnimationState::Braking => {
      if speed < 40 { speed = 40; }
      350
    },
    _ => 18,
  };
  mem.drag_vec = mem.normalized_velocity_vec * Fixed16::from_raw_i16(((di as i32 * speed as i32) / -160) as i16);
  if mem.ski_jump_upwards_movement_frames != 0 {
    mem.drag_vec.y = Fixed16::zero();
  }
}

fn ski_jump_crash_20086(mem: &mut Mem) {
  mem.animation_state = AnimationState::Crashed;
  mem.animation_state_counter = 0;
  // mem.byte_5051d = 0;
  ski_jump_update_state(mem, SkiJumpState::Crashed);
}

fn clamp_value(val: i16, min: i16, max: i16) -> i16 {
  if val < min { return min; }
  if val > max { return max; }
  val
}

fn ski_jump_read_inputs_1f414(mem: &mut Mem) {
  mem.left_right_input = 0; // -3 to 3
  mem.up_down_input = 0; // -3 to 3
  mem.enter_pressed = false;
}

#[allow(dead_code)]
fn ski_jump_initialize_memory_1f2e8(mem: &mut Mem) {
  *mem = Mem::default();
  mem.next_jmptrack_segment = 0;
  mem.position_vec.x = Fixed32::from_raw_i32(0);
  mem.position_vec.z = Fixed32::from_raw_i32(0x1c0);
  mem.animation_state = AnimationState::Start;
  // mem.byte_504a4 = 0;
  let _ax = { // sub_20186
    mem.next_jmptrack_segment = 0;
    load_next_geometry_segment(mem, mem.jmptrack_cur_segment);
    let ax = calc_player_segment_square(mem, mem.position_vec.clone());
    let si = ax;
    calculate_surface_normal(mem, ax);
    // extra code that doesn't seem to do anything
    // if si < 3 {
    //   mem.ski_jump_position_vec.z.raw() as i16 - (mem.word_505aa[si as usize].z.raw() + mem.ski_jump_jmptrack_segment_buffer_5056e.o_8) * mem.ski_jump_surface_normal_vec_maybe_5050c.z
    // }
    si
  };
  mem.segment_square = calc_player_segment_square(mem, mem.position_vec.clone());
  load_next_geometry_segment(mem, mem.jmptrack_cur_segment);
  /* mem.word_5050a = */calculate_surface_normal(mem, mem.segment_square);
  mem.ski_flight_angle = (next_xor_rng_value(mem) as i16 % 0x2000) + 0x1000;
  ski_jump_camera_control_20688(mem);
  ski_jump_update_state(mem, SkiJumpState::NotStarted);
}

fn ski_jump_update_state(mem: &mut Mem, ax: SkiJumpState) {
  if mem.state != ax {
    mem.state = ax;
    mem.state_frame_count = 0;
  }
}

fn ski_jump_camera_control_20688(mem: &mut Mem) {
  let var_8 = &JMPTRACK_DATA[mem.jmptrack_cur_segment as usize];

  let word_50554: Fixed16;
  let word_50556: Fixed16;
  let word_50536: Fixed16;
  let word_50538: Wrapping<u16>;
  let mut word_5053c: Vec3Fixed32;

  if mem.state == SkiJumpState::NotStarted || (mem.state == SkiJumpState::Jumping && mem.state_frame_count < 8) {
    word_5053c = mem.position_vec.clone();
    word_50536 = Fixed16::from_raw_i16(0x800 - var_8.o_e.0 as i16);
    word_50538 = Wrapping(0x7e00) + var_8.o_e;
    word_50554 = Fixed16::from_raw_i16(0x20);
    word_50556 = Fixed16::from_raw_i16(0x200);
  } else if mem.jmptrack_cur_segment < 12 {
    word_5053c = mem.position_vec.clone();
    if mem.jmptrack_cur_segment < 15 {
      word_5053c.x = Fixed32::from_raw_i32(clamp_dword(word_5053c.x.raw(), -0x190, 0x190));
    }
    word_50536 = Fixed16::from_raw_i16(0x200 - var_8.o_e.0 as i16);
    word_50538 = Wrapping(0x7c00) + var_8.o_e;
    word_50554 = Fixed16::from_raw_i16(0x20);
    word_50556 = Fixed16::from_raw_i16(0x140);
  } else if mem.sub_state != SkiJumpSubState::Landed {
    word_5053c = mem.position_vec.clone();
    if mem.jmptrack_cur_segment < 15 {
      word_5053c.x = Fixed32::from_raw_i32(clamp_dword(word_5053c.x.raw(), -0x190, 0x190));
    }
    word_50536 = Fixed16::from_raw_i16(0xc00 - var_8.o_e.0 as i16);
    word_50538 = Wrapping(0x7c00) + var_8.o_e;
    word_50554 = Fixed16::from_raw_i16(0);
    word_50556 = Fixed16::from_raw_i16(0x140);
  } else {
    word_5053c = mem.position_vec.clone();
    word_50536 = Fixed16::from_raw_i16(-(var_8.o_e.0 as i16));
    word_50538 = Wrapping(0x8000) + var_8.o_e;
    word_50554 = Fixed16::from_raw_i16(0);
    word_50556 = Fixed16::from_raw_i16(0x140);
  }

  let var_2 = scaled_sine(word_50538, word_50556) + word_50554;
  let var_4 = scaled_cosine(word_50538, word_50556);

  let mut var_14 = Vec3Fixed16::default();
  let var_e = Vec3Fixed16 {
    x: Fixed16::from_raw_i16(word_5053c.x.raw() as i16 - mem.dword_503fa.x.raw() as i16),
    y: Fixed16::from_raw_i16(word_5053c.y.raw() as i16 - mem.dword_503fa.y.raw() as i16) + var_2,
    z: Fixed16::from_raw_i16(word_5053c.z.raw() as i16 - mem.dword_503fa.z.raw() as i16) + var_4,
  };

  if !mem.word_50534 {
    var_14 = var_e;
    mem.word_50548 = Vec3Fixed16::default();
  } else {
    let si = if mem.jmptrack_cur_segment < 10 { 4 } else { 3 };
    mem.word_50548.x = sub_1f7b0(var_14.x, var_e.x, mem.word_50548.x, si);
    mem.word_50548.y = sub_1f7b0(var_14.y, var_e.y, mem.word_50548.y, si);
    mem.word_50548.z = sub_1f7b0(var_14.z, var_e.z, mem.word_50548.z, si);
    var_14 = var_14 + mem.word_50548;
  }
  mem.dword_503fa = mem.dword_503fa + var_14.to_fixed32();
  let var_e = Vec3Fixed16 {
    x: word_50536 - mem.word_50414.x,
    y: -mem.word_50414.y,
    z: Fixed16::zero(),
  };
  if !mem.word_50534 {
    mem.word_50414.x = mem.word_50414.x + var_e.x;
    mem.word_50414.y = mem.word_50414.y + var_e.y;
    mem.word_5054e = Vec3Fixed16::default();
  } else {
    mem.word_5054e.x = sub_1f7b0(Fixed16::zero(), var_e.x, mem.word_5054e.x, 4);
    mem.word_5054e.y = sub_1f7b0(Fixed16::zero(), var_e.y, mem.word_5054e.y, 4);
    mem.word_5054e.z = sub_1f7b0(Fixed16::zero(), Fixed16::zero(), mem.word_5054e.z, 4);
    mem.word_50414 = mem.word_50414 + mem.word_5054e;
  }
  mem.word_50534 = true;
}


fn sub_1f7b0(ax: Fixed16, dx: Fixed16, bx: Fixed16, arg0: i16) -> Fixed16 {
  bx + Fixed16::from_raw_i16(((dx.0 - arg0 * bx.0 - ax.0) * 2) / (arg0 * arg0))
  // arg=4: bx/2 + (dx-ax)/8 -> equilibrium at (dx-ax)/4
  // arg=3: 1/3*bx + 2/9*(dx-ax) -> equilibrium at (dx-ax)/3
}

fn scaled_sine(angle: Wrapping<u16>, length: Fixed16) -> Fixed16 {
  sine(angle) * length
}

fn scaled_cosine(angle: Wrapping<u16>, length: Fixed16) -> Fixed16 {
  sine(angle + Wrapping(0x4000)) * length
}

fn sine(angle: Wrapping<u16>) -> Fixed16 {
  const SINE_LOOKUP_TABLE: [i16; 65] = [
       0x0,  0x324,  0x648,  0x96B, 0x0C8C, 0x0FAB, 0x12C8, 0x15E2, 0x18F9, 0x1C0C, 0x1F1A, 0x2224, 0x2528, 0x2827, 0x2B1F, 0x2E11,
    0x30FC, 0x33DF, 0x36BA, 0x398D, 0x3C57, 0x3F17, 0x41CE, 0x447B, 0x471D, 0x49B4, 0x4C40, 0x4EC0, 0x5134, 0x539B, 0x55F6, 0x5843,
    0x5A82, 0x5CB4, 0x5ED7, 0x60EC, 0x62F2, 0x64E9, 0x66D0, 0x68A7, 0x6A6E, 0x6C24, 0x6DCA, 0x6F5F, 0x70E3, 0x7255, 0x73B6, 0x7505,
    0x7642, 0x776C, 0x7885, 0x798A, 0x7A7D, 0x7B5D, 0x7C2A, 0x7CE4, 0x7D8A, 0x7E1E, 0x7E9D, 0x7F0A, 0x7F62, 0x7FA7, 0x7FD9, 0x7FF6,
    0x7FFF,
  ];
  let ax = if angle.0 & 0x4000 != 0 { !angle.0 } else { angle.0 } & 0x7fff;
  let i = (ax as usize) >> 8;
  let diff = SINE_LOOKUP_TABLE[i+1] - SINE_LOOKUP_TABLE[i];
  let ax = (((ax as u32 & 0xff) * diff as u32) >> 8) as i16 + SINE_LOOKUP_TABLE[i];
  Fixed16::from_raw_i16(if angle.0 < 0x8000 { ax } else { -ax })
}

fn clamp_dword(value: i32, min: i32, max: i32) -> i32 {
  if value < min { return min; }
  if value > max { return max; }
  value
}

fn next_xor_rng_value(mem: &mut Mem) -> u16 {
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
  let bx = RNG_TABLE[mem.counter_529f6 as usize & 0x3f];
  mem.counter_529f6 = mem.counter_529f6.wrapping_add(1);
  mem.counter_529f8 = mem.counter_529f8.wrapping_add(0x1b65);
  (mem.counter_529f8 ^ bx) & 0x7fff
}

fn calculate_surface_normal(mem: &mut Mem, ax: u16) -> u16 {
  let ax = ax as usize;

  let var_1c = (mem.segment_relative_position.x - mem.cur_track_segment_vertices_relative[ax].x).raw() as i32 * (mem.next_track_segment_vertices_relative[ax + 1].z - mem.cur_track_segment_vertices_relative[ax].z).raw() as i32;
  let dxax = (mem.segment_relative_position.z - mem.cur_track_segment_vertices_relative[ax].z).raw() as i32 * (mem.next_track_segment_vertices_relative[ax + 1].x - mem.cur_track_segment_vertices_relative[ax].x).raw() as i32;

  let var_6;
  let cx;
  let di;
  if dxax > var_1c { // determine which triangle the player is on
    var_6 = 1;
    cx = mem.next_track_segment_vertices_relative[ax + 1];
    di = mem.cur_track_segment_vertices_relative[ax + 1];
  } else {
    var_6 = 0;
    cx = mem.next_track_segment_vertices_relative[ax];
    di = mem.next_track_segment_vertices_relative[ax + 1];
  }
  let var_e = cx - mem.cur_track_segment_vertices_relative[ax];
  let var_14 = di - mem.cur_track_segment_vertices_relative[ax];

  let var_1a = Vec3Fixed16::cross_product_scaled_2b8b8(var_e, var_14);
  mem.surface_normal_vec = Vec3Fixed16::normalize(var_1a).0;

  return var_6;
}

fn calc_player_segment_square(mem: &mut Mem, bx_position_vec: Vec3Fixed32) -> u16 {
  { // sub_1F62C
    mem.segment_relative_position = bx_position_vec.to_fixed16() - mem.jmptrack_segment_buffer.o_0.to_fixed16();
  }
  if mem.segment_relative_position.z.raw() < -0x60 {
    return 0x80; // in front of current segment
  }
  if /*(mem.word_505a4.x * mem.ski_jump_unk_x_diff_divide_z_diff_505a0) +*/ mem.jmptrack_next_segment_relative_z <= mem.segment_relative_position.z {
    return 0x81; // behind current segment
  }
  for i in 0..4 {
    let line_delta_z = mem.next_track_segment_vertices_relative[i].z - mem.cur_track_segment_vertices_relative[i].z;
    let line_delta_x = mem.next_track_segment_vertices_relative[i].x - mem.cur_track_segment_vertices_relative[i].x;
    if line_delta_x.abs() > line_delta_z.abs() {
      let dz_by_dx = line_delta_z / line_delta_x;
      let si = (mem.segment_relative_position.x * dz_by_dx) + (mem.cur_track_segment_vertices_relative[i].z - mem.cur_track_segment_vertices_relative[i].x * dz_by_dx);
      if dz_by_dx >= Fixed16::zero() {
        if si >= mem.segment_relative_position.z {
          return if i == 0 { 0x82 } else { i as u16 - 1 }; // to the right of line i (in square i-1..i)
        }
      } else {
        if si <= mem.segment_relative_position.z {
          return if i == 0 { 0x82 } else { i as u16 - 1 }; // to the right of line i (in square i-1..i)
        }
      }
    } else if line_delta_x.abs() <= line_delta_z.abs() {
      let dx_by_dz = line_delta_x / line_delta_z;
      let line_x_at_position_z = (mem.segment_relative_position.z * dx_by_dz) + (mem.cur_track_segment_vertices_relative[i].x - mem.cur_track_segment_vertices_relative[i].z * dx_by_dz);
      if mem.segment_relative_position.x >= line_x_at_position_z {
        return if i == 0 { 0x82 } else { i as u16 - 1 }; // to the right of line i (in square i-1..i)
      }
    }
  }
  return 0x83; // to the left of all lines
}

fn load_next_geometry_segment(mem: &mut Mem, stack_ax: u16) {
  const SEGMENT_VERTEX_DATA: [Fixed16; 32] = [
    Fixed16(-0x1E0), Fixed16(    0),   Fixed16(-0x1A0), Fixed16(0),   Fixed16(0x1A0), Fixed16(0),   Fixed16( 0x1E0), Fixed16(    0),  // ramp segment
    Fixed16(-0xC80), Fixed16(0x180),   Fixed16(-0x640), Fixed16(0),   Fixed16(0x640), Fixed16(0),   Fixed16( 0xC80), Fixed16(0x180),  // ground segment
    Fixed16( -0xC0), Fixed16(    0),   Fixed16( -0x80), Fixed16(0),   Fixed16( 0x80), Fixed16(0),   Fixed16(  0xC0), Fixed16(    0),  // start segment
    Fixed16(-0x280), Fixed16(    0),   Fixed16(-0x240), Fixed16(0),   Fixed16(0x240), Fixed16(0),   Fixed16( 0x280), Fixed16(    0),  // ramp end segment
  ];

  if stack_ax.wrapping_sub(mem.next_jmptrack_segment) != 0xffff {
    let start_index = stack_ax as usize;
    mem.jmptrack_segment_buffer = JMPTRACK_DATA[start_index].clone();
    mem.jmptrack_next_segment_ptr = start_index + 1;
    { // load_jmptrack_relative_vertex_data
      let esdi = mem.jmptrack_segment_buffer.o11_segment_type as usize * 8;
      for i in 0..4 {
        mem.cur_track_segment_vertices_relative[i].x = -SEGMENT_VERTEX_DATA[esdi + 2*i];
        mem.cur_track_segment_vertices_relative[i].y = SEGMENT_VERTEX_DATA[esdi + 2*i + 1];
        mem.cur_track_segment_vertices_relative[i].z = Fixed16::from_raw_i16(0);
      }
      let esbx = mem.jmptrack_next_segment_ptr;
      let next_segment_relative_y = JMPTRACK_DATA[esbx].o_0.y.to_fixed16() - mem.jmptrack_segment_buffer.o_0.y.to_fixed16();
      let esdi = JMPTRACK_DATA[esbx].o11_segment_type as usize * 8;
      for i in 0..4 {
        mem.next_track_segment_vertices_relative[i].x = -SEGMENT_VERTEX_DATA[esdi + 2*i];
        mem.next_track_segment_vertices_relative[i].y = SEGMENT_VERTEX_DATA[esdi + 2*i + 1] + next_segment_relative_y;
        mem.next_track_segment_vertices_relative[i].z = mem.jmptrack_segment_buffer.oc_next_segment_relative_z;
      }
    }
    // let denom = mem.next_track_segment_vertices_relative[3].x - mem.next_track_segment_vertices_relative[0].x;
    // let numer = mem.next_track_segment_vertices_relative[3].z - mem.next_track_segment_vertices_relative[0].z;  // guaranteed to be 0?
    // mem.ski_jump_unk_x_diff_divide_z_diff_505a0 = numer / denom;
    mem.jmptrack_next_segment_relative_z = mem.next_track_segment_vertices_relative[0].z /*- mem.next_track_segment_vertices_relative[0].x * mem.ski_jump_unk_x_diff_divide_z_diff_505a0*/;
    mem.next_jmptrack_segment = stack_ax + 1;
  }
}

