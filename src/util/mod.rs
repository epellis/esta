pub mod fold;
pub mod stack;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn bool_to_i64(cond: bool) -> i64 {
    if cond {
        1
    } else {
        0
    }
}

pub fn i64_to_bool(cond: i64) -> bool {
    cond == 1
}

pub fn string_hash32(s: String) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish() % (2_u64.pow(32))
}
