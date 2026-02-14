use ::criterion::{Criterion, criterion_group, criterion_main};
use ::my_lib::random::RandomNumberGenerator;

pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("random", |b| {
    // #[allow(unused_mut)]
    let mut rng: RandomNumberGenerator = Default::default();

    b.iter(|| rng.range(1_f32..10_000_000_f32))
  });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
