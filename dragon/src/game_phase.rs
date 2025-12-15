use ::bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GamePhase {
  #[default]
  Flapping,
  GameOver,
  MainMenu,
}
