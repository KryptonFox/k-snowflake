use crate::constants::*;
use crate::context::{sequence_increment, CONTEXT};
use crate::utils::time_since_epoch;
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::Ordering;

/// Snowflake structure
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Snowflake {
    /// Timestamp in ms since snowflake epoch start
    pub timestamp: u64,
    /// Instance of the snowflake (see schema in readme)
    pub instance: u16,
    /// Sequence of the snowflake (see schema in readme)
    pub sequence: u16,
}

impl Snowflake {
    /// New snowflake
    pub fn new(timestamp: u64, instance: u16, sequence: u16) -> Self {
        Self {
            timestamp,
            instance,
            sequence,
        }
    }

    /// New snowflake based on context values
    pub fn from_context() -> Self {
        let snowflake = Snowflake::new(
            time_since_epoch(CONTEXT.epoch_start.load(Ordering::Relaxed)),
            CONTEXT.instance.load(Ordering::Relaxed),
            CONTEXT.sequence.load(Ordering::Relaxed),
        );
        if CONTEXT.sequence_autoincrement.load(Ordering::Relaxed) {
            sequence_increment();
        }
        snowflake
    }

    /// Return UNIX timestamp in ms of snowflake
    pub fn get_unix_timestamp(&self) -> u64 {
        CONTEXT.epoch_start.load(Ordering::Relaxed) + self.timestamp
    }

    /// Return decimal value of snowflake
    pub fn to_decimal(&self) -> i64 {
        self.produce()
    }

    /// Return binary string of snowflake
    pub fn to_bin(&self) -> String {
        format!("{:b}", self.produce()).to_string()
    }

    fn produce(&self) -> i64 {
        let mut snowflake = self.timestamp as i64;
        snowflake <<= INSTANCE_BYTES;
        snowflake |= (self.instance & 0x3FF) as i64;
        snowflake <<= SEQUENCE_BYTES;
        snowflake |= (self.sequence & 0xFFF) as i64;
        snowflake
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_decimal())
    }
}

impl fmt::Binary for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_bin())
    }
}

#[derive(Debug)]
pub struct SnowflakeParseError;

impl FromStr for Snowflake {
    type Err = SnowflakeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut snowflake: i64 = s.parse().map_err(|_| SnowflakeParseError)?;

        let sequence = (snowflake & 0xFFF) as u16;
        snowflake >>= SEQUENCE_BYTES;
        let instance = (snowflake & 0x3FF) as u16;
        snowflake >>= INSTANCE_BYTES;
        let timestamp = snowflake as u64;

        Ok(Self {
            timestamp,
            instance,
            sequence,
        })
    }
}
