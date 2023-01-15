use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

pub fn sploosh_benchmark(c: &mut Criterion){
    c.bench_function("sploosh 2", |b| b.iter(|| sploosh(black_box(8),black_box(9),black_box(10))));
}

criterion_group!(benches, sploosh_benchmark);
criterion_main!(benches);

/* remember that each benchmark has a time dependency. A warm up period of 3 seconds and a measurement period
of 5 seconds. The fewer things running on the computer at the time the better. 
Don't benchmark on public CI servers - they're very noisy! 