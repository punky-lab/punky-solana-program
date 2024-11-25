use crate::config::*;

pub fn clamp(value: u16, min: u16, max: u16) -> u16 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn stat_clamp(value: u16) -> u16 {
    clamp(value, MIN_STAT_VALUE, MAX_STAT_VALUE)
}
