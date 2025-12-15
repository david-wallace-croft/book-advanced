use ::bevy::prelude::*;
use ::bevy::state::state::FreelyMutableState;

pub struct GameStatePlugin<T> {
  pub game_end_state: T,
  pub game_start_state: T,
  pub menu_state: T,
}

impl<T: FreelyMutableState + FromWorld + States> Plugin for GameStatePlugin<T> {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.init_state::<T>();
  }
}
