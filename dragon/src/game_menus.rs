use super::menu_assets::MenuAssets;
use super::menu_element::MenuElement;
use super::menu_resource::MenuResource;
use ::bevy::state::state::FreelyMutableState;
use ::bevy::{app::AppExit, prelude::*};

pub fn setup<T>(
  state: Res<State<T>>,
  mut commands: Commands,
  menu_resource: Res<MenuResource<T>>,
  asset_server: Res<AssetServer>,
) where
  T: States + FromWorld + FreelyMutableState,
{
  let menu_assets: MenuAssets = MenuAssets {
    main_menu: asset_server.load("main_menu.png"),
    game_over: asset_server.load("game_over.png"),
  };

  let current_state: &T = state.get();

  let menu_graphic: Handle<Image> = {
    if menu_resource.menu_state == *current_state {
      menu_assets.main_menu.clone()
    } else if menu_resource.game_end_state == *current_state {
      menu_assets.game_over.clone()
    } else {
      panic!("Unkown menu state");
    }
  };

  commands.spawn(Camera2d::default()).insert(MenuElement);

  commands
    .spawn((
      Sprite {
        image: menu_graphic,
        ..default()
      },
      Transform::from_xyz(0., 0., 1.),
    ))
    .insert(MenuElement);
}
