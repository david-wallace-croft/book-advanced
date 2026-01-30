use ::bevy::prelude::*;

#[expect(dead_code)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GamePhase {
  Cpu,
  End,
  GameOver,
  #[default]
  Player,
  MainMenu,
  Start,
}
