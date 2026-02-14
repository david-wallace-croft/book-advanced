use ::bevy::prelude::*;

#[derive(Clone, Copy, Resource)]
pub struct Scores {
  pub cpu: usize,
  pub player: usize,
}
