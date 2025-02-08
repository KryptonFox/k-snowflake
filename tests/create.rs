use k_snowflake::Snowflake;

#[test]
fn create_snowflake() {
    let snowflake = Snowflake::from_timestamp(450207707886, 598, 1073);
    assert_eq!(snowflake.to_decimal().unwrap(), 1888307990019531825)
}
