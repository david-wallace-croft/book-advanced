use super::cleanup;
use super::game_menus;
use super::menu_assets;
use super::menu_element::MenuElement;
use super::menu_resource::MenuResource;
use ::bevy::prelude::*;
use ::bevy::state::state::FreelyMutableState;

pub struct GameStatePlugin<T> {
  pub game_end_state: T,
  pub game_start_state: T,
  pub menu_state: T,
}

impl<T> Plugin for GameStatePlugin<T>
where
  T: Copy + FreelyMutableState + FromWorld + States,
{
  fn build(
    &self,
    app: &mut App,
  ) {
    app.init_state::<T>();

    app.add_systems(Startup, menu_assets::setup_menus);

    let start = MenuResource {
      game_end_state: self.game_end_state,
      game_start_state: self.game_start_state,
      menu_state: self.menu_state,
    };

    app.insert_resource(start);

    app.add_systems(OnEnter(self.menu_state), game_menus::setup::<T>);

    app.add_systems(
      Update,
      game_menus::run::<T>.run_if(in_state(self.menu_state)),
    );

    app.add_systems(OnExit(self.menu_state), cleanup::<MenuElement>);

    app.add_systems(OnEnter(self.game_end_state), game_menus::setup::<T>);

    app.add_systems(
      Update,
      game_menus::run::<T>.run_if(in_state(self.game_end_state)),
    );

    app.add_systems(OnExit(self.game_end_state), cleanup::<MenuElement>);
  }
}
