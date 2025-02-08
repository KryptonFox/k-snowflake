use crate::constants::*;
use crate::utils::{sys_time_millis, time_since_epoch};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default)]
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

    pub fn to_decimal(&self) -> Result<u64, String> {
        Ok(u64::from_str_radix(self.to_bin()?.as_str(), 2).unwrap())
    }

    pub fn to_bin(&self) -> Result<String, String> {
        let timestamp_bin = format!("{:b}", self.timestamp);
        if timestamp_bin.chars().count() > TIMESTAMP_BYTES {
            return Err("Timestamp too long".to_string());
        }

        let instance_bin = format!("{:b}", self.instance);
        if instance_bin.chars().count() > INSTANCE_BYTES {
            return Err("Timestamp too long".to_string());
        }

        let sequence_bin = format!("{:b}", self.sequence);
        if sequence_bin.chars().count() > SEQUENCE_BYTES {
            return Err("Timestamp too long".to_string());
        }

        Ok(format!(
            "0{:0>41}{:0>10}{:0>12}",
            timestamp_bin, instance_bin, sequence_bin
        ))
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
        let snowflake: u64 = s.parse().map_err(|_| SnowflakeParseError)?;
        let mut snowflake_bin = format!("{:b}", snowflake);
        snowflake_bin = format!("0{:0>63}", snowflake_bin);
        dbg!(&snowflake_bin);
        let timestamp =
            u64::from_str_radix(&snowflake_bin[..42], 2).map_err(|_| SnowflakeParseError)?;
        let instance =
            u16::from_str_radix(&snowflake_bin[42..52], 2).map_err(|_| SnowflakeParseError)?;
        let sequence =
            u16::from_str_radix(&snowflake_bin[52..], 2).map_err(|_| SnowflakeParseError)?;
        Ok(Self {
            timestamp,
            instance,
            sequence,
        })
    }
}
