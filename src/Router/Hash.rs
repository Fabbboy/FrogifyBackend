#![allow(non_snake_case)]
#![allow(dead_code)]
use std::time::SystemTime;
use chrono::{DateTime, Local};
use sha2::Sha256;
use sha2::Digest;
pub(crate) fn generateHashAdv(usermail: String, current_time: SystemTime) -> String {
    let mut hasher = Sha256::new();
    hasher.update(usermail);
    hasher.update(current_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros().to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}
pub(crate) fn generateHashRandom(string: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string);
    hasher.update(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros().to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}

pub(crate) fn systimeToString(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}


