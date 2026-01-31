use super::menu_assets::MenuAssets;
use super::menu_element::MenuElement;
use super::menu_resource::MenuResource;
use ::bevy::state::state::FreelyMutableState;
use ::bevy::{app::AppExit, prelude::*};

pub fn run<T>(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut exit: MessageWriter<AppExit>,
  current_state: Res<State<T>>,
  mut state: ResMut<NextState<T>>,
  menu_state: Res<MenuResource<T>>,
) where
  T: States + FromWorld + FreelyMutableState,
{
  let current_state: T = current_state.get().clone();

  if current_state == menu_state.menu_state {
    if keyboard.just_pressed(KeyCode::KeyP) {
      state.set(menu_state.game_start_state.clone());
    } else if keyboard.just_pressed(KeyCode::KeyQ) {
      exit.write(AppExit::Success);
    }
  } else if current_state == menu_state.game_end_state {
    if keyboard.just_pressed(KeyCode::KeyM) {
      state.set(menu_state.menu_state.clone());
    } else if keyboard.just_pressed(KeyCode::KeyQ) {
      exit.write(AppExit::Success);
    }
  }
}

pub fn setup<T>(
  state: Res<State<T>>,
  mut commands: Commands,
  menu_resource: Res<MenuResource<T>>,
  asset_server: Res<AssetServer>,
) where
  T: States + FromWorld + FreelyMutableState,
{
  let menu_assets: MenuAssets = MenuAssets {
    main_menu: asset_server.load("main-menu.png"),
    game_over: asset_server.load("game-over.png"),
  };

  let current_state: &T = state.get();

  let menu_graphic: Handle<Image> = {
    if menu_resource.menu_state == *current_state {
      menu_assets.main_menu.clone()
    } else if menu_resource.game_end_state == *current_state {
      menu_assets.game_over.clone()
    } else {
      panic!("Unknown menu state");
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
