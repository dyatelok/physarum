const PI: f32 = std::f32::consts::PI;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

pub const TRAIL: f32 = 100.0;

pub const GRADIENT_BOUND: f32 = 1000.0;
pub const PHEROMONE_LIMIT: f32 = 1000000.0;

pub const TARGET_FPS: u64 = 60;
<<<<<<< HEAD
pub const TPF: u64 = 1;
=======
pub const TPF: u64 = 10;
>>>>>>> refs/remotes/origin/main

pub const WOBBLE: f32 = PI * 0.04;
pub const FORCED_ROT: f32 = 0.0; //PI / 60.0;

pub const AGENTS_N: usize = 40000;

pub const ROT_M_AV: f32 = PI / 7.0;
pub const ROT_M_DIS: f32 = PI / 32.0;

pub const SPEED_AV: f32 = 1.7;
pub const SPEED_DIS: f32 = 0.5;

pub const ROT_S_AV: f32 = PI / 2.0;
pub const ROT_S_DIS: f32 = PI / 12.0;

pub const DIS_S_AV: f32 = 4.0;
pub const DIS_S_DIS: f32 = 1.3;

pub const C_LEVEL_AV: f32 = 10000.0;
pub const C_LEVEL_DIS: f32 = 1000.0;

pub const C_DURATION_AV: u16 = 300;
pub const C_DURATION_DIS: u16 = 50;

const DECAY: f32 = 1.0 - 0.002;

const RELATIV_C: f32 = 1.0;
const RELATIV_S: f32 = 2.0;
const RELATIV_Z: f32 = 50.0;

const SUM: f32 = 4.0 * (RELATIV_C + RELATIV_S) + RELATIV_Z;

const C: f32 = RELATIV_C / SUM * DECAY;
const S: f32 = RELATIV_S / SUM * DECAY;
const Z: f32 = RELATIV_Z / SUM * DECAY;

pub const DIFFUSION: [[f32; 3]; 3] = [[C, S, C], [S, Z, S], [C, S, C]];

pub const VIRIDIS: [[u8; 4]; 512] = [
    [0x44, 0x01, 0x54, 0xff],
    [0x44, 0x02, 0x55, 0xff],
    [0x44, 0x03, 0x56, 0xff],
    [0x44, 0x04, 0x56, 0xff],
    [0x45, 0x04, 0x57, 0xff],
    [0x45, 0x05, 0x58, 0xff],
    [0x45, 0x06, 0x58, 0xff],
    [0x45, 0x06, 0x59, 0xff],
    [0x45, 0x07, 0x5a, 0xff],
    [0x45, 0x08, 0x5b, 0xff],
    [0x45, 0x09, 0x5b, 0xff],
    [0x45, 0x09, 0x5c, 0xff],
    [0x46, 0x0a, 0x5d, 0xff],
    [0x46, 0x0b, 0x5d, 0xff],
    [0x46, 0x0b, 0x5e, 0xff],
    [0x46, 0x0c, 0x5f, 0xff],
    [0x46, 0x0d, 0x5f, 0xff],
    [0x46, 0x0e, 0x60, 0xff],
    [0x46, 0x0e, 0x61, 0xff],
    [0x46, 0x0f, 0x61, 0xff],
    [0x47, 0x10, 0x62, 0xff],
    [0x47, 0x10, 0x63, 0xff],
    [0x47, 0x11, 0x63, 0xff],
    [0x47, 0x12, 0x64, 0xff],
    [0x47, 0x13, 0x65, 0xff],
    [0x47, 0x13, 0x66, 0xff],
    [0x47, 0x14, 0x66, 0xff],
    [0x47, 0x15, 0x67, 0xff],
    [0x48, 0x15, 0x68, 0xff],
    [0x48, 0x16, 0x68, 0xff],
    [0x48, 0x17, 0x69, 0xff],
    [0x48, 0x18, 0x6a, 0xff],
    [0x48, 0x18, 0x6a, 0xff],
    [0x48, 0x19, 0x6b, 0xff],
    [0x48, 0x1a, 0x6b, 0xff],
    [0x48, 0x1a, 0x6c, 0xff],
    [0x48, 0x1b, 0x6c, 0xff],
    [0x48, 0x1c, 0x6d, 0xff],
    [0x48, 0x1c, 0x6d, 0xff],
    [0x48, 0x1d, 0x6e, 0xff],
    [0x48, 0x1e, 0x6f, 0xff],
    [0x48, 0x1e, 0x6f, 0xff],
    [0x48, 0x1f, 0x70, 0xff],
    [0x48, 0x20, 0x70, 0xff],
    [0x48, 0x20, 0x71, 0xff],
    [0x48, 0x21, 0x71, 0xff],
    [0x48, 0x22, 0x72, 0xff],
    [0x48, 0x22, 0x72, 0xff],
    [0x47, 0x23, 0x73, 0xff],
    [0x47, 0x23, 0x73, 0xff],
    [0x47, 0x24, 0x74, 0xff],
    [0x47, 0x25, 0x74, 0xff],
    [0x47, 0x25, 0x75, 0xff],
    [0x47, 0x26, 0x75, 0xff],
    [0x47, 0x27, 0x76, 0xff],
    [0x47, 0x27, 0x76, 0xff],
    [0x47, 0x28, 0x77, 0xff],
    [0x47, 0x29, 0x78, 0xff],
    [0x47, 0x29, 0x78, 0xff],
    [0x47, 0x2a, 0x79, 0xff],
    [0x47, 0x2b, 0x79, 0xff],
    [0x47, 0x2b, 0x7a, 0xff],
    [0x47, 0x2c, 0x7a, 0xff],
    [0x47, 0x2d, 0x7b, 0xff],
    [0x47, 0x2d, 0x7b, 0xff],
    [0x47, 0x2e, 0x7c, 0xff],
    [0x47, 0x2e, 0x7c, 0xff],
    [0x46, 0x2f, 0x7c, 0xff],
    [0x46, 0x30, 0x7d, 0xff],
    [0x46, 0x30, 0x7d, 0xff],
    [0x46, 0x31, 0x7d, 0xff],
    [0x46, 0x31, 0x7e, 0xff],
    [0x46, 0x32, 0x7e, 0xff],
    [0x46, 0x33, 0x7e, 0xff],
    [0x45, 0x33, 0x7f, 0xff],
    [0x45, 0x34, 0x7f, 0xff],
    [0x45, 0x34, 0x7f, 0xff],
    [0x45, 0x35, 0x80, 0xff],
    [0x45, 0x36, 0x80, 0xff],
    [0x45, 0x36, 0x80, 0xff],
    [0x44, 0x37, 0x81, 0xff],
    [0x44, 0x37, 0x81, 0xff],
    [0x44, 0x38, 0x81, 0xff],
    [0x44, 0x39, 0x82, 0xff],
    [0x44, 0x39, 0x82, 0xff],
    [0x44, 0x3a, 0x82, 0xff],
    [0x43, 0x3a, 0x83, 0xff],
    [0x43, 0x3b, 0x83, 0xff],
    [0x43, 0x3c, 0x83, 0xff],
    [0x43, 0x3c, 0x84, 0xff],
    [0x43, 0x3d, 0x84, 0xff],
    [0x43, 0x3d, 0x84, 0xff],
    [0x43, 0x3e, 0x85, 0xff],
    [0x42, 0x3f, 0x85, 0xff],
    [0x42, 0x3f, 0x85, 0xff],
    [0x42, 0x40, 0x86, 0xff],
    [0x42, 0x40, 0x86, 0xff],
    [0x42, 0x41, 0x86, 0xff],
    [0x41, 0x41, 0x86, 0xff],
    [0x41, 0x42, 0x87, 0xff],
    [0x41, 0x43, 0x87, 0xff],
    [0x41, 0x43, 0x87, 0xff],
    [0x41, 0x44, 0x87, 0xff],
    [0x40, 0x44, 0x87, 0xff],
    [0x40, 0x45, 0x87, 0xff],
    [0x40, 0x45, 0x87, 0xff],
    [0x40, 0x46, 0x88, 0xff],
    [0x3f, 0x46, 0x88, 0xff],
    [0x3f, 0x47, 0x88, 0xff],
    [0x3f, 0x48, 0x88, 0xff],
    [0x3f, 0x48, 0x88, 0xff],
    [0x3f, 0x49, 0x88, 0xff],
    [0x3e, 0x49, 0x89, 0xff],
    [0x3e, 0x4a, 0x89, 0xff],
    [0x3e, 0x4a, 0x89, 0xff],
    [0x3e, 0x4b, 0x89, 0xff],
    [0x3e, 0x4c, 0x89, 0xff],
    [0x3d, 0x4c, 0x89, 0xff],
    [0x3d, 0x4d, 0x8a, 0xff],
    [0x3d, 0x4d, 0x8a, 0xff],
    [0x3d, 0x4e, 0x8a, 0xff],
    [0x3c, 0x4e, 0x8a, 0xff],
    [0x3c, 0x4f, 0x8a, 0xff],
    [0x3c, 0x4f, 0x8a, 0xff],
    [0x3c, 0x50, 0x8a, 0xff],
    [0x3c, 0x51, 0x8b, 0xff],
    [0x3b, 0x51, 0x8b, 0xff],
    [0x3b, 0x52, 0x8b, 0xff],
    [0x3b, 0x52, 0x8b, 0xff],
    [0x3b, 0x53, 0x8b, 0xff],
    [0x3a, 0x53, 0x8b, 0xff],
    [0x3a, 0x54, 0x8b, 0xff],
    [0x3a, 0x54, 0x8b, 0xff],
    [0x3a, 0x55, 0x8b, 0xff],
    [0x39, 0x55, 0x8b, 0xff],
    [0x39, 0x56, 0x8b, 0xff],
    [0x39, 0x57, 0x8c, 0xff],
    [0x39, 0x57, 0x8c, 0xff],
    [0x38, 0x58, 0x8c, 0xff],
    [0x38, 0x58, 0x8c, 0xff],
    [0x38, 0x59, 0x8c, 0xff],
    [0x38, 0x59, 0x8c, 0xff],
    [0x37, 0x5a, 0x8c, 0xff],
    [0x37, 0x5a, 0x8c, 0xff],
    [0x37, 0x5b, 0x8c, 0xff],
    [0x37, 0x5b, 0x8c, 0xff],
    [0x36, 0x5c, 0x8c, 0xff],
    [0x36, 0x5c, 0x8c, 0xff],
    [0x36, 0x5d, 0x8c, 0xff],
    [0x36, 0x5d, 0x8c, 0xff],
    [0x35, 0x5e, 0x8c, 0xff],
    [0x35, 0x5e, 0x8c, 0xff],
    [0x35, 0x5f, 0x8d, 0xff],
    [0x35, 0x60, 0x8d, 0xff],
    [0x34, 0x60, 0x8d, 0xff],
    [0x34, 0x61, 0x8d, 0xff],
    [0x34, 0x61, 0x8d, 0xff],
    [0x34, 0x62, 0x8d, 0xff],
    [0x33, 0x62, 0x8d, 0xff],
    [0x33, 0x63, 0x8d, 0xff],
    [0x33, 0x63, 0x8d, 0xff],
    [0x33, 0x64, 0x8d, 0xff],
    [0x32, 0x64, 0x8d, 0xff],
    [0x32, 0x65, 0x8d, 0xff],
    [0x32, 0x65, 0x8d, 0xff],
    [0x32, 0x66, 0x8d, 0xff],
    [0x32, 0x66, 0x8d, 0xff],
    [0x31, 0x67, 0x8d, 0xff],
    [0x31, 0x67, 0x8d, 0xff],
    [0x31, 0x67, 0x8d, 0xff],
    [0x31, 0x68, 0x8d, 0xff],
    [0x30, 0x68, 0x8d, 0xff],
    [0x30, 0x69, 0x8d, 0xff],
    [0x30, 0x69, 0x8d, 0xff],
    [0x30, 0x6a, 0x8d, 0xff],
    [0x30, 0x6a, 0x8d, 0xff],
    [0x2f, 0x6b, 0x8e, 0xff],
    [0x2f, 0x6b, 0x8e, 0xff],
    [0x2f, 0x6c, 0x8e, 0xff],
    [0x2f, 0x6c, 0x8e, 0xff],
    [0x2f, 0x6d, 0x8e, 0xff],
    [0x2e, 0x6d, 0x8e, 0xff],
    [0x2e, 0x6e, 0x8e, 0xff],
    [0x2e, 0x6e, 0x8e, 0xff],
    [0x2e, 0x6e, 0x8e, 0xff],
    [0x2d, 0x6f, 0x8e, 0xff],
    [0x2d, 0x6f, 0x8e, 0xff],
    [0x2d, 0x70, 0x8e, 0xff],
    [0x2d, 0x70, 0x8e, 0xff],
    [0x2d, 0x71, 0x8e, 0xff],
    [0x2c, 0x71, 0x8e, 0xff],
    [0x2c, 0x72, 0x8e, 0xff],
    [0x2c, 0x72, 0x8e, 0xff],
    [0x2c, 0x73, 0x8e, 0xff],
    [0x2c, 0x73, 0x8e, 0xff],
    [0x2b, 0x74, 0x8e, 0xff],
    [0x2b, 0x74, 0x8e, 0xff],
    [0x2b, 0x75, 0x8e, 0xff],
    [0x2b, 0x75, 0x8e, 0xff],
    [0x2b, 0x76, 0x8e, 0xff],
    [0x2a, 0x76, 0x8e, 0xff],
    [0x2a, 0x76, 0x8e, 0xff],
    [0x2a, 0x77, 0x8e, 0xff],
    [0x2a, 0x77, 0x8e, 0xff],
    [0x2a, 0x78, 0x8e, 0xff],
    [0x29, 0x78, 0x8e, 0xff],
    [0x29, 0x79, 0x8e, 0xff],
    [0x29, 0x79, 0x8e, 0xff],
    [0x29, 0x7a, 0x8e, 0xff],
    [0x29, 0x7a, 0x8e, 0xff],
    [0x29, 0x7b, 0x8e, 0xff],
    [0x28, 0x7b, 0x8e, 0xff],
    [0x28, 0x7c, 0x8e, 0xff],
    [0x28, 0x7c, 0x8e, 0xff],
    [0x28, 0x7d, 0x8e, 0xff],
    [0x28, 0x7d, 0x8e, 0xff],
    [0x27, 0x7d, 0x8e, 0xff],
    [0x27, 0x7e, 0x8e, 0xff],
    [0x27, 0x7e, 0x8e, 0xff],
    [0x27, 0x7f, 0x8e, 0xff],
    [0x27, 0x7f, 0x8e, 0xff],
    [0x26, 0x80, 0x8e, 0xff],
    [0x26, 0x80, 0x8e, 0xff],
    [0x26, 0x81, 0x8e, 0xff],
    [0x26, 0x81, 0x8e, 0xff],
    [0x26, 0x82, 0x8e, 0xff],
    [0x26, 0x82, 0x8e, 0xff],
    [0x25, 0x83, 0x8e, 0xff],
    [0x25, 0x83, 0x8e, 0xff],
    [0x25, 0x84, 0x8e, 0xff],
    [0x25, 0x84, 0x8e, 0xff],
    [0x25, 0x85, 0x8e, 0xff],
    [0x25, 0x85, 0x8d, 0xff],
    [0x25, 0x85, 0x8d, 0xff],
    [0x24, 0x86, 0x8d, 0xff],
    [0x24, 0x86, 0x8d, 0xff],
    [0x24, 0x87, 0x8d, 0xff],
    [0x24, 0x87, 0x8d, 0xff],
    [0x24, 0x88, 0x8d, 0xff],
    [0x24, 0x88, 0x8d, 0xff],
    [0x23, 0x89, 0x8d, 0xff],
    [0x23, 0x89, 0x8d, 0xff],
    [0x23, 0x8a, 0x8d, 0xff],
    [0x23, 0x8a, 0x8d, 0xff],
    [0x23, 0x8b, 0x8d, 0xff],
    [0x23, 0x8b, 0x8d, 0xff],
    [0x22, 0x8c, 0x8d, 0xff],
    [0x22, 0x8c, 0x8d, 0xff],
    [0x22, 0x8c, 0x8c, 0xff],
    [0x22, 0x8d, 0x8c, 0xff],
    [0x22, 0x8d, 0x8c, 0xff],
    [0x22, 0x8e, 0x8c, 0xff],
    [0x22, 0x8e, 0x8c, 0xff],
    [0x21, 0x8f, 0x8c, 0xff],
    [0x21, 0x8f, 0x8c, 0xff],
    [0x21, 0x90, 0x8c, 0xff],
    [0x21, 0x90, 0x8c, 0xff],
    [0x21, 0x91, 0x8c, 0xff],
    [0x21, 0x91, 0x8c, 0xff],
    [0x21, 0x92, 0x8c, 0xff],
    [0x21, 0x92, 0x8b, 0xff],
    [0x21, 0x93, 0x8b, 0xff],
    [0x21, 0x93, 0x8b, 0xff],
    [0x21, 0x94, 0x8b, 0xff],
    [0x20, 0x94, 0x8b, 0xff],
    [0x20, 0x94, 0x8b, 0xff],
    [0x20, 0x95, 0x8b, 0xff],
    [0x20, 0x95, 0x8b, 0xff],
    [0x20, 0x96, 0x8a, 0xff],
    [0x20, 0x96, 0x8a, 0xff],
    [0x20, 0x97, 0x8a, 0xff],
    [0x20, 0x97, 0x8a, 0xff],
    [0x20, 0x98, 0x8a, 0xff],
    [0x20, 0x98, 0x8a, 0xff],
    [0x20, 0x99, 0x8a, 0xff],
    [0x20, 0x99, 0x8a, 0xff],
    [0x20, 0x9a, 0x89, 0xff],
    [0x20, 0x9a, 0x89, 0xff],
    [0x20, 0x9b, 0x89, 0xff],
    [0x20, 0x9b, 0x89, 0xff],
    [0x1f, 0x9b, 0x89, 0xff],
    [0x1f, 0x9c, 0x89, 0xff],
    [0x1f, 0x9c, 0x89, 0xff],
    [0x1f, 0x9d, 0x89, 0xff],
    [0x1f, 0x9d, 0x88, 0xff],
    [0x1f, 0x9e, 0x88, 0xff],
    [0x1f, 0x9e, 0x88, 0xff],
    [0x1f, 0x9f, 0x88, 0xff],
    [0x1f, 0x9f, 0x88, 0xff],
    [0x1f, 0xa0, 0x88, 0xff],
    [0x20, 0xa0, 0x87, 0xff],
    [0x20, 0xa1, 0x87, 0xff],
    [0x20, 0xa1, 0x87, 0xff],
    [0x21, 0xa2, 0x87, 0xff],
    [0x21, 0xa2, 0x86, 0xff],
    [0x21, 0xa3, 0x86, 0xff],
    [0x21, 0xa3, 0x86, 0xff],
    [0x22, 0xa3, 0x86, 0xff],
    [0x22, 0xa4, 0x85, 0xff],
    [0x22, 0xa4, 0x85, 0xff],
    [0x23, 0xa5, 0x85, 0xff],
    [0x23, 0xa5, 0x85, 0xff],
    [0x23, 0xa6, 0x84, 0xff],
    [0x23, 0xa6, 0x84, 0xff],
    [0x24, 0xa7, 0x84, 0xff],
    [0x24, 0xa7, 0x84, 0xff],
    [0x24, 0xa8, 0x83, 0xff],
    [0x24, 0xa8, 0x83, 0xff],
    [0x25, 0xa9, 0x83, 0xff],
    [0x25, 0xa9, 0x83, 0xff],
    [0x25, 0xaa, 0x82, 0xff],
    [0x26, 0xaa, 0x82, 0xff],
    [0x26, 0xaa, 0x82, 0xff],
    [0x26, 0xab, 0x82, 0xff],
    [0x26, 0xab, 0x81, 0xff],
    [0x27, 0xac, 0x81, 0xff],
    [0x27, 0xac, 0x81, 0xff],
    [0x27, 0xad, 0x81, 0xff],
    [0x28, 0xad, 0x80, 0xff],
    [0x28, 0xae, 0x80, 0xff],
    [0x28, 0xae, 0x80, 0xff],
    [0x29, 0xaf, 0x7f, 0xff],
    [0x2a, 0xaf, 0x7f, 0xff],
    [0x2a, 0xb0, 0x7f, 0xff],
    [0x2b, 0xb0, 0x7e, 0xff],
    [0x2c, 0xb0, 0x7e, 0xff],
    [0x2c, 0xb1, 0x7e, 0xff],
    [0x2d, 0xb1, 0x7d, 0xff],
    [0x2e, 0xb2, 0x7d, 0xff],
    [0x2f, 0xb2, 0x7c, 0xff],
    [0x2f, 0xb3, 0x7c, 0xff],
    [0x30, 0xb3, 0x7c, 0xff],
    [0x31, 0xb3, 0x7b, 0xff],
    [0x31, 0xb4, 0x7b, 0xff],
    [0x32, 0xb4, 0x7b, 0xff],
    [0x33, 0xb5, 0x7a, 0xff],
    [0x33, 0xb5, 0x7a, 0xff],
    [0x34, 0xb6, 0x79, 0xff],
    [0x35, 0xb6, 0x79, 0xff],
    [0x35, 0xb7, 0x79, 0xff],
    [0x36, 0xb7, 0x78, 0xff],
    [0x37, 0xb7, 0x78, 0xff],
    [0x37, 0xb8, 0x78, 0xff],
    [0x38, 0xb8, 0x77, 0xff],
    [0x39, 0xb9, 0x77, 0xff],
    [0x3a, 0xb9, 0x76, 0xff],
    [0x3a, 0xba, 0x76, 0xff],
    [0x3b, 0xba, 0x76, 0xff],
    [0x3c, 0xba, 0x75, 0xff],
    [0x3c, 0xbb, 0x75, 0xff],
    [0x3d, 0xbb, 0x75, 0xff],
    [0x3e, 0xbc, 0x74, 0xff],
    [0x3e, 0xbc, 0x74, 0xff],
    [0x3f, 0xbd, 0x73, 0xff],
    [0x40, 0xbd, 0x73, 0xff],
    [0x41, 0xbd, 0x72, 0xff],
    [0x42, 0xbe, 0x72, 0xff],
    [0x43, 0xbe, 0x71, 0xff],
    [0x44, 0xbf, 0x71, 0xff],
    [0x45, 0xbf, 0x70, 0xff],
    [0x46, 0xbf, 0x6f, 0xff],
    [0x47, 0xc0, 0x6f, 0xff],
    [0x48, 0xc0, 0x6e, 0xff],
    [0x49, 0xc1, 0x6e, 0xff],
    [0x4a, 0xc1, 0x6d, 0xff],
    [0x4b, 0xc1, 0x6d, 0xff],
    [0x4c, 0xc2, 0x6c, 0xff],
    [0x4d, 0xc2, 0x6c, 0xff],
    [0x4e, 0xc3, 0x6b, 0xff],
    [0x4f, 0xc3, 0x6b, 0xff],
    [0x50, 0xc4, 0x6a, 0xff],
    [0x51, 0xc4, 0x6a, 0xff],
    [0x52, 0xc4, 0x69, 0xff],
    [0x53, 0xc5, 0x69, 0xff],
    [0x54, 0xc5, 0x68, 0xff],
    [0x55, 0xc6, 0x68, 0xff],
    [0x56, 0xc6, 0x67, 0xff],
    [0x57, 0xc6, 0x66, 0xff],
    [0x58, 0xc7, 0x66, 0xff],
    [0x59, 0xc7, 0x65, 0xff],
    [0x5a, 0xc8, 0x65, 0xff],
    [0x5b, 0xc8, 0x64, 0xff],
    [0x5c, 0xc8, 0x64, 0xff],
    [0x5d, 0xc9, 0x63, 0xff],
    [0x5e, 0xc9, 0x63, 0xff],
    [0x5f, 0xc9, 0x62, 0xff],
    [0x60, 0xca, 0x61, 0xff],
    [0x61, 0xca, 0x60, 0xff],
    [0x62, 0xca, 0x60, 0xff],
    [0x63, 0xcb, 0x5f, 0xff],
    [0x65, 0xcb, 0x5e, 0xff],
    [0x66, 0xcb, 0x5e, 0xff],
    [0x67, 0xcc, 0x5d, 0xff],
    [0x68, 0xcc, 0x5c, 0xff],
    [0x69, 0xcc, 0x5b, 0xff],
    [0x6a, 0xcd, 0x5b, 0xff],
    [0x6b, 0xcd, 0x5a, 0xff],
    [0x6d, 0xcd, 0x59, 0xff],
    [0x6e, 0xce, 0x59, 0xff],
    [0x6f, 0xce, 0x58, 0xff],
    [0x70, 0xce, 0x57, 0xff],
    [0x71, 0xce, 0x56, 0xff],
    [0x72, 0xcf, 0x56, 0xff],
    [0x74, 0xcf, 0x55, 0xff],
    [0x75, 0xcf, 0x54, 0xff],
    [0x76, 0xd0, 0x54, 0xff],
    [0x77, 0xd0, 0x53, 0xff],
    [0x78, 0xd0, 0x52, 0xff],
    [0x79, 0xd1, 0x51, 0xff],
    [0x7a, 0xd1, 0x51, 0xff],
    [0x7c, 0xd1, 0x50, 0xff],
    [0x7d, 0xd2, 0x4f, 0xff],
    [0x7e, 0xd2, 0x4f, 0xff],
    [0x7f, 0xd2, 0x4e, 0xff],
    [0x80, 0xd3, 0x4d, 0xff],
    [0x81, 0xd3, 0x4c, 0xff],
    [0x83, 0xd3, 0x4c, 0xff],
    [0x84, 0xd3, 0x4b, 0xff],
    [0x85, 0xd4, 0x4a, 0xff],
    [0x86, 0xd4, 0x49, 0xff],
    [0x88, 0xd4, 0x48, 0xff],
    [0x89, 0xd5, 0x48, 0xff],
    [0x8a, 0xd5, 0x47, 0xff],
    [0x8c, 0xd5, 0x46, 0xff],
    [0x8d, 0xd5, 0x45, 0xff],
    [0x8e, 0xd6, 0x44, 0xff],
    [0x8f, 0xd6, 0x43, 0xff],
    [0x91, 0xd6, 0x43, 0xff],
    [0x92, 0xd7, 0x42, 0xff],
    [0x93, 0xd7, 0x41, 0xff],
    [0x95, 0xd7, 0x40, 0xff],
    [0x96, 0xd7, 0x3f, 0xff],
    [0x97, 0xd8, 0x3f, 0xff],
    [0x98, 0xd8, 0x3e, 0xff],
    [0x9a, 0xd8, 0x3d, 0xff],
    [0x9b, 0xd8, 0x3c, 0xff],
    [0x9c, 0xd9, 0x3b, 0xff],
    [0x9e, 0xd9, 0x3b, 0xff],
    [0x9f, 0xd9, 0x3a, 0xff],
    [0xa0, 0xda, 0x39, 0xff],
    [0xa1, 0xda, 0x38, 0xff],
    [0xa3, 0xda, 0x37, 0xff],
    [0xa4, 0xda, 0x36, 0xff],
    [0xa5, 0xdb, 0x36, 0xff],
    [0xa7, 0xdb, 0x35, 0xff],
    [0xa8, 0xdb, 0x34, 0xff],
    [0xa9, 0xdc, 0x33, 0xff],
    [0xaa, 0xdc, 0x32, 0xff],
    [0xac, 0xdc, 0x32, 0xff],
    [0xad, 0xdc, 0x31, 0xff],
    [0xae, 0xdc, 0x30, 0xff],
    [0xb0, 0xdd, 0x2f, 0xff],
    [0xb1, 0xdd, 0x2f, 0xff],
    [0xb2, 0xdd, 0x2e, 0xff],
    [0xb4, 0xdd, 0x2d, 0xff],
    [0xb5, 0xdd, 0x2c, 0xff],
    [0xb6, 0xde, 0x2c, 0xff],
    [0xb7, 0xde, 0x2b, 0xff],
    [0xb9, 0xde, 0x2a, 0xff],
    [0xba, 0xde, 0x29, 0xff],
    [0xbb, 0xde, 0x29, 0xff],
    [0xbd, 0xdf, 0x28, 0xff],
    [0xbe, 0xdf, 0x27, 0xff],
    [0xbf, 0xdf, 0x26, 0xff],
    [0xc1, 0xdf, 0x26, 0xff],
    [0xc2, 0xdf, 0x25, 0xff],
    [0xc3, 0xdf, 0x24, 0xff],
    [0xc5, 0xe0, 0x23, 0xff],
    [0xc6, 0xe0, 0x23, 0xff],
    [0xc7, 0xe0, 0x22, 0xff],
    [0xc9, 0xe0, 0x21, 0xff],
    [0xca, 0xe0, 0x20, 0xff],
    [0xcb, 0xe1, 0x20, 0xff],
    [0xcc, 0xe1, 0x1f, 0xff],
    [0xce, 0xe1, 0x1e, 0xff],
    [0xcf, 0xe1, 0x1d, 0xff],
    [0xd0, 0xe1, 0x1d, 0xff],
    [0xd2, 0xe2, 0x1c, 0xff],
    [0xd3, 0xe2, 0x1b, 0xff],
    [0xd4, 0xe2, 0x1a, 0xff],
    [0xd6, 0xe2, 0x1a, 0xff],
    [0xd7, 0xe2, 0x1b, 0xff],
    [0xd8, 0xe2, 0x1b, 0xff],
    [0xd9, 0xe3, 0x1b, 0xff],
    [0xdb, 0xe3, 0x1c, 0xff],
    [0xdc, 0xe3, 0x1c, 0xff],
    [0xdd, 0xe3, 0x1c, 0xff],
    [0xde, 0xe3, 0x1d, 0xff],
    [0xe0, 0xe3, 0x1d, 0xff],
    [0xe1, 0xe3, 0x1d, 0xff],
    [0xe2, 0xe4, 0x1e, 0xff],
    [0xe3, 0xe4, 0x1e, 0xff],
    [0xe5, 0xe4, 0x1e, 0xff],
    [0xe6, 0xe4, 0x1f, 0xff],
    [0xe7, 0xe4, 0x1f, 0xff],
    [0xe8, 0xe4, 0x1f, 0xff],
    [0xea, 0xe5, 0x20, 0xff],
    [0xeb, 0xe5, 0x20, 0xff],
    [0xec, 0xe5, 0x20, 0xff],
    [0xed, 0xe5, 0x21, 0xff],
    [0xef, 0xe5, 0x21, 0xff],
    [0xf0, 0xe5, 0x21, 0xff],
    [0xf1, 0xe6, 0x22, 0xff],
    [0xf2, 0xe6, 0x22, 0xff],
    [0xf4, 0xe6, 0x22, 0xff],
    [0xf5, 0xe6, 0x23, 0xff],
    [0xf6, 0xe6, 0x23, 0xff],
    [0xf7, 0xe6, 0x23, 0xff],
    [0xf9, 0xe6, 0x24, 0xff],
    [0xfa, 0xe7, 0x24, 0xff],
    [0xfb, 0xe7, 0x24, 0xff],
    [0xfc, 0xe7, 0x25, 0xff],
];
