use k_snowflake::Snowflake;

#[test]
fn test_parse_snowflake() {
    let snowflake = Snowflake::new(450207707886, 783, 1807);
    let snowflake_string = snowflake.to_string();
    
    assert_eq!(snowflake_string.parse::<Snowflake>().unwrap(), snowflake)
}