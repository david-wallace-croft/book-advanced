use ::bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GamePhase {
  Cpu,
  End,
  GameOver,
  Player,
  MainMenu,
  #[default]
  Start,
}
