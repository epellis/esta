pub mod stack;
pub mod fold;

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
