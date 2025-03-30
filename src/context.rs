use crate::constants::{INSTANCE_BYTES, SEQUENCE_BYTES, TWITTER_EPOCH};
use crate::epoch::Epoch;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static CONTEXT: Lazy<Mutex<Context>> = Lazy::new(|| Mutex::new(Context::new()));

pub struct Context {
    pub epoch: Epoch,
    pub sequence: u16,
    pub instance: u16,
    pub sequence_autoincrement: bool,
}

impl Context {
    fn new() -> Self {
        Self {
            epoch: TWITTER_EPOCH,
            instance: (std::process::id() % 2u32.pow(INSTANCE_BYTES)) as u16,
            sequence: 0,
            sequence_autoincrement: true,
        }
    }

    pub fn increment(&mut self) {
        self.sequence = (self.sequence + 1) % 2u16.pow(SEQUENCE_BYTES)
    }
}

pub fn set_instance(instance: u16) {
    let mut ctx = CONTEXT.lock().unwrap();
    ctx.instance = instance
}

pub fn set_sequence(sequence: u16) {
    let mut ctx = CONTEXT.lock().unwrap();
    ctx.sequence = sequence
}

/// Set sequence number autoincrement on every snowflake creation from context.
/// Default: true
pub fn set_sequence_autoincrement(sequence_autoincrement: bool) {
    let mut ctx = CONTEXT.lock().unwrap();
    ctx.sequence_autoincrement = sequence_autoincrement
}

pub fn set_epoch(epoch: Epoch) {
    let mut ctx = CONTEXT.lock().unwrap();
    ctx.epoch = epoch
}
