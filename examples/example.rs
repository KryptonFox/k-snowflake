use k_snowflake::{create_snowflake, set_epoch, Snowflake, DISCORD_EPOCH_START};

fn main() {
    // Create snowflake
    let snowflake = create_snowflake();
    println!("{}", snowflake);

    // change epoch
    set_epoch(DISCORD_EPOCH_START);
    let snowflake = create_snowflake();
    println!("{}", snowflake);

    // get UNIX timestamp of discord snowflake
    println!(
        "{}",
        "1301619246953926811"
            .parse::<Snowflake>()
            .unwrap()
            .get_unix_timestamp()
    );
}
