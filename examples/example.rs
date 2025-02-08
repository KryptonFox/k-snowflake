use k_snowflake::Snowflake;

fn main() {
    // Create snowflake
    let snowflake = Snowflake::new(123, 123);

    // convert to decimal
    let decimal_snowflake = snowflake.to_decimal().unwrap();

    // supports display
    println!("{}", snowflake);
}
