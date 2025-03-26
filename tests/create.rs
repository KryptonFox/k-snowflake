use k_snowflake::Snowflake;

#[test]
fn test_create_snowflake() {
    let snowflake = Snowflake::new(450207707886, 783, 1807);
    assert_eq!(snowflake.to_decimal(), 1888307990020290319)
}
