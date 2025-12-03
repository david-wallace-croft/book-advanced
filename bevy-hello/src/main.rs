use self::dragon::Dragon;
use ::bevy::prelude::*;

mod dragon;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, Dragon::movement)
    .run();
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  commands.spawn(Camera2d::default());

  let dragon_image: Handle<Image> = asset_server.load("dragon.png");

  commands
    .spawn(Sprite::from_image(dragon_image))
    .insert(Dragon);
}
