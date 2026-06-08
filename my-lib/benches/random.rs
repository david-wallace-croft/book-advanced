use ::criterion::{Bencher, Criterion, criterion_group, criterion_main};
use ::my_lib::random::RandomNumberGenerator;

pub fn criterion_benchmark(criterion: &mut Criterion) {
  criterion.bench_function("random", |bencher: &mut Bencher<'_>| {
    let mut rng: RandomNumberGenerator = Default::default();

    bencher.iter(move || rng.range(1_f32..10_000_000_f32))
  });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
