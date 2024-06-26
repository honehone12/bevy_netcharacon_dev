use bevy::utils::{SystemTime, Uuid};

pub const DEV_SERVER_TICK_RATE: f32 = 20.0;
pub const DEV_SERVER_TICK_DELTA: f32 = 1.0 / DEV_SERVER_TICK_RATE;
pub const DEV_NETWORK_TICK_RATE: u16 = 10;
pub const DEV_NETWORK_TICK_DELTA: f32 = 1.0 / (DEV_NETWORK_TICK_RATE as f32); 
pub const DEV_NETWORK_TICK_DELTA64: f64 = 1.0 / (DEV_NETWORK_TICK_RATE as f64); 

pub const DEV_SERVER_LISTEN_PORT: u16 = 5000;
pub const DEV_SERVER_MAX_CLIENTS: usize = 10;

pub const DEV_CLIENT_TIME_OUT_SEC: i32 = 15;
pub const DEV_TOKEN_EXPIRE_SEC: u64 = 300;

pub const DEV_MAX_UPDATE_SNAPSHOT_SIZE: usize = 2560;
pub const DEV_MAX_SNAPSHOT_SIZE: usize = 64;

pub const BASE_SPEED: f32 = 10.0;
pub const BASE_ANGULAR_SPEED: f32 = 25.0; 

// (network delata time / local delta time * base speed * local delta time)^2
// bevy's fixed update = 64hz
// DEV_NETWORK_TICK_DELTA / 0.0156 * BASE_SPEED * 0.0156 = 0.9999
pub const TRANSLATION_ERROR_THRESHOLD: f32 = 1.0;
pub const ROTATION_ERROR_THRESHOLD: f32 = 1.0;
// 1sec / network tick
pub const PREDICTION_ERROR_COUNT_THRESHOLD: u32 = 10;

pub const DISTANCE_CULLING_THREASHOLD: f32 = 100.0;

pub const PHYSICS_FIXED_TICK_RATE: f32 = 64.0;
pub const PHYSICS_FIXED_TICK_RATE64: f64 = 64.0;
pub const PHYSICS_FIXED_TICK_DELTA: f32 = 1.0 / PHYSICS_FIXED_TICK_RATE;
pub const PHYSICS_SUBSTEP: f32 = 12.0; 

pub fn get_dev_protocol_id() -> u64 {
    if cfg!(debug_assertions) {
        0x655ea1eecade99ad
    } else {
        panic!("do not use dev protocol id");
    }
}

pub fn get_dev_private_key() -> [u8; 32] {
    if cfg!(debug_assertions) {
        [
            0x78, 0xe8, 0xbb, 0x30, 0xa2, 0xb, 0x11, 0xf2, 
            0xaa, 0xf6, 0x61, 0x3e, 0xa3, 0xb9, 0xf2, 0x9a, 
            0x53, 0x1f, 0xa7, 0x63, 0x27, 0x27, 0x53, 0x69, 
            0xe4, 0xb2, 0x34, 0x54, 0x15, 0x48, 0x2c, 0xaf
        ]
    } else {
        panic!("do not use dev private key");
    }
}

pub fn get_dev_client_id() -> u64 {
    if cfg!(debug_assertions) {
        SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
    } else {
        panic!("do not use dev client id");
    }
}

pub fn get_dev_user_data() -> [u8; 256] {
    if cfg!(debug_assertions) {
        // this will be session id generated by backend service
        let mut user_data = [0u8; 256];
        user_data[0..16].copy_from_slice(Uuid::new_v4().as_bytes());
        user_data
    } else {
        panic!("do not use dev user data")
    }
}

