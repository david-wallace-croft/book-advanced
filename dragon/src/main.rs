use self::dragon::Dragon;
use self::dragon_element::DragonElement;
use self::game_phase::GamePhase;
use self::obstacle::Obstacle;
use ::bevy::prelude::*;
use ::bevy::window::WindowResolution;
use ::my_lib::add_phase;
use ::my_lib::bevy_assets::asset_manager::AssetManager;
use ::my_lib::bevy_assets::asset_store::LoadedAssets;
use ::my_lib::bevy_assets::asset_store::{AssetResource, AssetStore};
use ::my_lib::game_state_plugin::GameStatePlugin;
use ::my_lib::random::RandomNumberGenerator;
use ::my_lib::random_plugin::RandomPlugin;
use ::my_lib::spawn_image;

mod dragon;
mod dragon_element;
mod game_phase;
mod obstacle;

fn main() -> ::anyhow::Result<()> {
  let resolution: WindowResolution = WindowResolution::new(1024, 768);

  let primary_window: Option<Window> = Some(Window {
    // position: WindowPosition::Centered(MonitorSelection::Primary),
    position: WindowPosition::At(IVec2::new(300, 0)),
    resolution,
    title: "Flappy Dragon - Bevy Edition".to_string(),
    ..default()
  });

  let window_plugin: WindowPlugin = WindowPlugin {
    primary_window,
    ..default()
  };

  let mut app: App = App::new();

  add_phase!(
    app,
    GamePhase,
    GamePhase::Flapping,
    start => [ setup ],
    run => [ gravity, flap, clamp, move_walls, hit_wall ],
    exit => [ ::my_lib::cleanup::<DragonElement> ]
  );

  let asset_manager: AssetManager = AssetManager::default()
    .add_image("dragon", "dragon-52x45.png")?
    .add_image("wall", "wall-32x32.png")?;

  app
    .add_plugins(DefaultPlugins.set(window_plugin))
    .add_plugins(RandomPlugin)
    .add_plugins(asset_manager)
    .add_plugins(GameStatePlugin::<GamePhase> {
      game_end_state: GamePhase::GameOver,
      game_start_state: GamePhase::Flapping,
      menu_state: GamePhase::MainMenu,
    })
    .run();

  Ok(())
}

fn build_wall(
  assets: &AssetStore,
  commands: &mut Commands,
  loaded_assets: &LoadedAssets,
  gap_y: isize,
) {
  for y in -12..=12 {
    if y < gap_y - 4 || y > gap_y + 4 {
      spawn_image!(
        assets,
        commands,
        "wall",
        512.,
        y as f32 * 32.,
        1.,
        &loaded_assets,
        Obstacle,
        DragonElement
      );
    }
  }
}

fn clamp(
  mut query: Query<&mut Transform, With<Dragon>>,
  mut state: ResMut<NextState<GamePhase>>,
) {
  if let Ok(mut transform) = query.single_mut() {
    if transform.translation.y > 384. {
      transform.translation.y = 384.;
    } else if transform.translation.y < -384. {
      state.set(GamePhase::GameOver)
    }
  }
}

fn flap(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut query: Query<&mut Dragon>,
) {
  if keyboard.pressed(KeyCode::Space)
    && let Ok(mut dragon) = query.single_mut()
  {
    dragon.gravity -= 0.24;
  }
}

fn gravity(mut query: Query<(&mut Dragon, &mut Transform)>) {
  if let Ok((mut dragon, mut transform)) = query.single_mut() {
    dragon.gravity += 0.04;

    transform.translation.y -= dragon.gravity;
  }
}

fn hit_wall(
  player: Query<&Transform, With<Dragon>>,
  walls: Query<&Transform, With<Obstacle>>,
  mut state: ResMut<NextState<GamePhase>>,
) {
  if let Ok(player) = player.single() {
    for wall in walls.iter() {
      let distance: f32 = player.translation.distance(wall.translation);

      if distance < 32. {
        state.set(GamePhase::GameOver)
      }
    }
  }
}

fn move_walls(
  assets: Res<AssetStore>,
  mut commands: Commands,
  mut query: Query<&mut Transform, With<Obstacle>>,
  delete: Query<Entity, With<Obstacle>>,
  loaded_assets: AssetResource,
  #[allow(unused_mut)] mut rng: ResMut<RandomNumberGenerator>,
) {
  let mut rebuild: bool = false;

  for mut transform in query.iter_mut() {
    transform.translation.x -= 4.;

    if transform.translation.x < -530. {
      rebuild = true;
    }
  }

  if rebuild {
    for entity in delete.iter() {
      commands.entity(entity).despawn();
    }

    build_wall(
      &assets,
      &mut commands,
      &loaded_assets,
      rng.range(-5..5) as isize,
    );
  }
}

fn setup(
  assets: Res<AssetStore>,
  mut commands: Commands,
  loaded_assets: AssetResource,
  #[allow(unused_mut)] mut rng: ResMut<RandomNumberGenerator>,
) {
  commands.spawn(Camera2d).insert(DragonElement);

  let dragon: Dragon = Dragon {
    gravity: 0.,
  };

  spawn_image!(
    assets,
    commands,
    "dragon",
    -490.,
    0.,
    1.,
    &loaded_assets,
    dragon,
    DragonElement
  );

  let gap_y: isize = rng.range(-5..5) as isize;

  build_wall(&assets, &mut commands, &loaded_assets, gap_y);
}
