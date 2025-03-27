use k_snowflake::create_snowflake;

#[test]
fn create_million_snowflakes_from_context() {
    for _i in 0..1_000_000 {
        create_snowflake().to_decimal();
    }
}