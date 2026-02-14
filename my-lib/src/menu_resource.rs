use ::bevy::prelude::*;

#[derive(Resource)]
pub struct MenuResource<T> {
  pub game_end_state: T,
  pub game_start_state: T,
  pub menu_state: T,
}
