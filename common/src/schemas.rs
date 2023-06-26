use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct UserDB {
    id: i64,
    email: String,
    username: String,
    password_hash: String,
    token: String,
    login_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Token {
    vec: Vec<String>
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserRegister {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub fn u8x4_to_u32_le(data: &[u8]) -> u32 {
    ((data[0] as u32) <<  0) +
    ((data[1] as u32) <<  8) +
    ((data[2] as u32) << 16) +
    ((data[3] as u32) << 24)
}


pub fn u32_to_u8x4_le(data: &[u32; 4]) -> [u8; 16] {
    let mut res = [0; 16];
    for i in 0..4 {
        res[4*i..][..4].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}
