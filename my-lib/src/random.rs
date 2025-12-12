use ::bevy::prelude::*;
use ::rand::Rng;
use ::rand::SeedableRng;
use ::rand::distr::Distribution;
use ::rand::distr::StandardUniform;
use ::rand::distr::uniform::SampleRange;
use ::rand::distr::uniform::SampleUniform;

#[cfg(
  all(
    not(feature = "pcg"),
    not(feature = "xorshift")
  )
)]
type RngCore = ::rand::prelude::StdRng;

#[cfg(feature = "pcg")]
type RngCore = ::rand_pcg::Pcg64Mcg;

#[cfg(feature = "xorshift")]
type RngCore = ::rand_xorshift::XorShiftRng;

#[derive(Resource)]
pub struct RandomNumberGenerator {
  rng: RngCore,
}

impl RandomNumberGenerator {
  pub fn next<T>(&mut self) -> T
  where
    StandardUniform: Distribution<T>,
  {
    self.rng.random()
  }

  pub fn range<T>(
    &mut self,
    range: impl SampleRange<T>,
  ) -> T
  where
    T: PartialOrd + SampleUniform,
  {
    self.rng.random_range(range)
  }

  pub fn seeded(seed: u64) -> Self {
    Self {
      rng: RngCore::seed_from_u64(seed),
    }
  }
}

impl Default for RandomNumberGenerator {
  fn default() -> Self {
    Self {
      rng: RngCore::from_os_rng(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_float() {
    let mut rng: RandomNumberGenerator = Default::default();

    for _ in 0..1_000 {
      let n: f32 = rng.range(-5_000.0f32..5_000.0f32);

      assert!(n.is_finite());

      assert!(n >= -5_000.);

      assert!(n <= 5_000.)
    }
  }

  #[test]
  fn test_next_types() {
    let mut rng: RandomNumberGenerator = Default::default();

    let _: i32 = rng.next();

    let _ = rng.next::<f32>();
  }

  #[test]
  fn test_range_bounds() {
    let mut rng: RandomNumberGenerator = Default::default();

    for _ in 0..1_000 {
      let n: u32 = rng.range(1..10);

      assert!(n >= 1);

      assert!(n < 10);
    }
  }

  #[test]
  fn test_reproducibility() {
    let mut rng = (
      RandomNumberGenerator::seeded(1),
      RandomNumberGenerator::seeded(1),
    );

    (0..1_000).for_each(|_| {
      assert_eq!(
        rng.0.range(u32::MIN..u32::MAX),
        rng.1.range(u32::MIN..u32::MAX),
      );
    })
  }
}
