use k_snowflake::Snowflake;

fn main() {
    // Create snowflake
    let snowflake = Snowflake::new(598, 1073);

    // convert to decimal
    let _decimal_snowflake = snowflake.to_decimal().unwrap();

    // supports display
    println!("{}", snowflake);
}
