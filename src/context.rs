use crate::constants::TWITTER_EPOCH_START;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU64, Ordering};
use std::sync::Arc;

pub static CONTEXT: Lazy<Arc<Context>> = Lazy::new(|| Arc::new(Context::new()));

pub struct Context {
    pub epoch_start: AtomicU64,
    pub sequence: AtomicU16,
    pub instance: AtomicU16,
    pub sequence_autoincrement: AtomicBool,
}

impl Context {
    fn new() -> Self {
        Self {
            epoch_start: AtomicU64::new(TWITTER_EPOCH_START),
            instance: AtomicU16::new((std::process::id() & 0x3FF) as u16),
            sequence: AtomicU16::new(0),
            sequence_autoincrement: AtomicBool::new(true),
        }
    }
}

pub fn sequence_increment() {
    CONTEXT.sequence.store(
        (CONTEXT.sequence.load(Ordering::Relaxed) + 1) & 0xFFF,
        Ordering::Relaxed,
    );
}

pub fn set_instance(instance: u16) {
    CONTEXT.instance.store(instance, Ordering::Relaxed);
}

pub fn set_sequence(sequence: u16) {
    CONTEXT.sequence.store(sequence, Ordering::Relaxed);
}

/// Set sequence number autoincrement on every snowflake creation from context.
/// Default: true
pub fn set_sequence_autoincrement(sequence_autoincrement: bool) {
    CONTEXT
        .sequence_autoincrement
        .store(sequence_autoincrement, Ordering::Relaxed);
}

pub fn set_epoch(epoch: u64) {
    // *CONTEXT.epoch_start.write().unwrap() = epoch;
    CONTEXT.epoch_start.store(epoch, Ordering::Relaxed);
}
