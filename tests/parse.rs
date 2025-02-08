use k_snowflake::Snowflake;

#[test]
fn parse_snowflake() {
    let snowflake = Snowflake::new(598, 1073);
    let snowflake_string = snowflake.to_string();

    assert_eq!(snowflake_string.parse::<Snowflake>().unwrap(), snowflake)
}