use super::RandomNumberGenerator;
use ::bevy::prelude::*;

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.insert_resource(RandomNumberGenerator::default());
  }
}
