use criterion::{criterion_group, criterion_main, Criterion};
use k_snowflake::create_snowflake;
use std::time::Duration;

fn bench(c: &mut Criterion) {
    c.bench_function("create decimal snowflake", |b| {
        b.iter(|| create_snowflake().to_decimal())
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(1000).measurement_time(Duration::from_secs(10));
    targets = bench
}
criterion_main!(benches);
