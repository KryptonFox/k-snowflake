mod snowflake;
pub mod utils;
mod constants;
mod context;

pub use snowflake::Snowflake;
pub use context::{
    set_sequence,
    set_instance,
    set_sequence_autoincrement,
    set_epoch
};
pub use constants::{TWITTER_EPOCH_START, DISCORD_EPOCH_START};

/// Basic creating snowflake using context
pub fn create_snowflake() -> Snowflake {
    Snowflake::from_context()
}