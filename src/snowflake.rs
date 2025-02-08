use crate::constants::*;
use crate::utils::{sys_time_millis, time_since_epoch};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
pub struct Snowflake {
    pub timestamp: u64,
    pub instance: u16,
    pub sequence: u16,
}

impl Snowflake {
    /// New snowflake based twitter`s epoch and local time
    pub fn new(instance: u16, sequence: u16) -> Self {
        Self {
            timestamp: time_since_epoch(TWITTER_EPOCH),
            instance,
            sequence,
        }
    }

    /// New snowflake based twitter`s epoch and given timestamp
    pub fn from_timestamp(timestamp: u64, instance: u16, sequence: u16) -> Self {
        Self {
            timestamp,
            instance,
            sequence,
        }
    }

    /// New snowflake based given epoch and local time.
    /// Provide UNIX time milliseconds since the chosen epoch
    pub fn with_epoch(epoch_millis: u64, instance: u16, sequence: u16) -> Self {
        Self {
            timestamp: time_since_epoch(epoch_millis),
            instance,
            sequence,
        }
    }

    /// New snowflake based given epoch and given timestamp.
    /// Provide UNIX time milliseconds since the chosen epoch.
    /// Provide UNIX timestamp
    pub fn with_epoch_and_timestamp(
        epoch_millis: u64,
        timestamp: u64,
        instance: u16,
        sequence: u16,
    ) -> Self {
        Self {
            timestamp: timestamp - epoch_millis,
            instance,
            sequence,
        }
    }

    /// Return millis since UNIX epoch using Twitter epoch and current timestamp
    pub fn get_unix_timestamp(&self) -> u64 {
        sys_time_millis() + TWITTER_EPOCH + self.timestamp
    }

    pub fn to_decimal(&self) -> Result<i64, String> {
        self.produce()
    }

    pub fn to_bin(&self) -> Result<String, String> {
        Ok(format!("{:b}", self.produce()?).to_string())
    }

    fn produce(&self) -> Result<i64, String> {
        if self.instance >= 2u16.pow(INSTANCE_BYTES) {
            return Err("Instance too long. Must be 10 bits".to_string());
        }
        if self.sequence >= 2u16.pow(SEQUENCE_BYTES) {
            return Err("Sequence too long. Must be 12 bits".to_string());
        }
        let mut snowflake = self.timestamp as i64;
        snowflake <<= INSTANCE_BYTES;
        snowflake += self.instance as i64;
        snowflake <<= SEQUENCE_BYTES;
        snowflake += self.sequence as i64;
        Ok(snowflake)
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_decimal().unwrap())
    }
}

impl fmt::Binary for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_bin().unwrap())
    }
}

#[derive(Debug)]
pub struct SnowflakeParseError;

impl FromStr for Snowflake {
    type Err = SnowflakeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut snowflake: i64 = s.parse().map_err(|_| SnowflakeParseError)?;

        let sequence = (snowflake & 0b1111_1111_1111) as u16;
        snowflake >>= SEQUENCE_BYTES;
        let instance = (snowflake & 0b11_1111_1111) as u16;
        snowflake >>= INSTANCE_BYTES;
        let timestamp = snowflake as u64;

        Ok(Self {
            timestamp,
            instance,
            sequence,
        })
    }
}
