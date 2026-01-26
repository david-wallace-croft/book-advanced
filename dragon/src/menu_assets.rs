use ::bevy::prelude::*;

#[derive(Resource)]
pub struct MenuAssets {
  pub game_over: Handle<Image>,
  pub main_menu: Handle<Image>,
}

pub fn setup_menus(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  let game_over: Handle<Image> = asset_server.load("game-over.png");

  let main_menu: Handle<Image> = asset_server.load("main-menu.png");

  let assets: MenuAssets = MenuAssets {
    game_over,
    main_menu,
  };

  commands.insert_resource(assets);
}
