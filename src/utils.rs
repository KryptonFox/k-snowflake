use crate::epoch::Epoch;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn sys_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Get time since given epoch
pub fn time_since_epoch(epoch: &Epoch) -> u64 {
    sys_time_millis() - epoch.start
}
