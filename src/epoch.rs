pub struct Epoch {
    /// UNIX timestamp in ms 
    pub start: u64,
}

impl Epoch {
    /// Create new epoch from start time in millis
    pub fn new(start: u64) -> Self {
        Self { start }
    }
}
