use self::dragon::Dragon;
use self::dragon_assets::DragonAssets;
use self::dragon_element::DragonElement;
use self::game_phase::GamePhase;
use self::game_state_plugin::GameStatePlugin;
use self::obstacle::Obstacle;
use ::bevy::prelude::*;
use ::bevy::window::WindowResolution;
use ::my_lib::random::RandomNumberGenerator;
use ::my_lib::random_plugin::RandomPlugin;

mod dragon;
mod dragon_assets;
mod dragon_element;
mod game_menus;
mod game_phase;
mod game_state_plugin;
mod menu_assets;
mod menu_element;
mod menu_resource;
mod obstacle;

macro_rules! add_phase {
  (
    $app:expr, $type:ty, $phase:expr,
    start => [ $($start:expr),* ],
    run => [ $($run:expr),* ],
    exit => [ $($exit:expr),* ]
  ) => {
    $($app.add_systems(
      bevy::prelude::OnEnter::<$type>($phase),
      $start);)*

    $($app.add_systems(
      bevy::prelude::Update,
      $run.run_if(in_state($phase))
    );)*

    $($app.add_systems(
      bevy::prelude::OnExit::<$type>($phase),
      $exit
    );)*
  }
}

fn main() {
  let resolution: WindowResolution = WindowResolution::new(1024, 768);

  let primary_window: Option<Window> = Some(Window {
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
    exit => [ cleanup::<DragonElement> ]
  );

  app
    .add_plugins(DefaultPlugins.set(window_plugin))
    .add_plugins(RandomPlugin)
    .add_plugins(GameStatePlugin::<GamePhase> {
      game_end_state: GamePhase::GameOver,
      game_start_state: GamePhase::Flapping,
      menu_state: GamePhase::MainMenu,
    })
    .run();
}

fn build_wall(
  commands: &mut Commands,
  wall_sprite: Handle<Image>,
  gap_y: isize,
) {
  for y in -12..=12 {
    if y < gap_y - 4 || y > gap_y + 4 {
      commands.spawn((
        Sprite::from_image(wall_sprite.clone()),
        Transform::from_xyz(512., y as f32 * 32., 1.),
        Obstacle,
        DragonElement,
      ));
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

pub fn cleanup<T>(
  query: Query<Entity, With<T>>,
  mut commands: Commands,
) where
  T: Component,
{
  query
    .iter()
    .for_each(|entity| commands.entity(entity).despawn());
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
  mut commands: Commands,
  mut query: Query<&mut Transform, With<Obstacle>>,
  delete: Query<Entity, With<Obstacle>>,
  assets: Res<DragonAssets>,
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
      &mut commands,
      assets.wall.clone(),
      rng.range(-5..5) as isize,
    );
  }
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  #[allow(unused_mut)] mut rng: ResMut<RandomNumberGenerator>,
) {
  let assets: DragonAssets = DragonAssets {
    dragon: asset_server.load("dragon-52x45.png"),
    wall: asset_server.load("wall-32x32.png"),
  };

  commands.spawn(Camera2d).insert(DragonElement);

  commands
    .spawn((
      Sprite::from_image(assets.dragon.clone()),
      Transform::from_xyz(-490., 0., 1.),
      Dragon {
        gravity: 0.,
      },
    ))
    .insert(DragonElement);

  build_wall(
    &mut commands,
    assets.wall.clone(),
    rng.range(-5..5) as isize,
  );

  commands.insert_resource(assets);
}
