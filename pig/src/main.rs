use self::game_assets::GameAssets;
use self::game_phase::GamePhase;
use self::hand_die::HandDie;
use self::hand_timer::HandTimer;
use self::scores::Scores;
use ::bevy::prelude::*;
use ::bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use ::my_lib::RandomNumberGenerator;
use ::my_lib::RandomPlugin;

mod game_assets;
mod game_phase;
mod hand_die;
mod hand_timer;
mod random;
mod scores;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin::default())
    .add_plugins(RandomPlugin)
    .add_systems(Startup, setup)
    .add_systems(EguiPrimaryContextPass, display_score)
    .init_state::<GamePhase>()
    .add_systems(
      EguiPrimaryContextPass,
      player.run_if(in_state(GamePhase::Player)),
    )
    .add_systems(Update, cpu.run_if(in_state(GamePhase::Cpu)))
    .run();
}

fn clear_die(
  hand_query: &Query<(Entity, &Sprite), With<HandDie>>,
  commands: &mut Commands,
) {
  hand_query
    .iter()
    .for_each(|(entity, _)| commands.entity(entity).despawn());
}

fn cpu(
  hand_query: Query<(Entity, &Sprite), With<HandDie>>,
  mut state: ResMut<NextState<GamePhase>>,
  mut scores: ResMut<Scores>,
  mut rng: ResMut<RandomNumberGenerator>,
  mut commands: Commands,
  assets: Res<GameAssets>,
  mut timer: ResMut<HandTimer>,
  time: Res<Time>,
) {
  timer.0.tick(time.delta());

  if timer.0.just_finished() {
    let hand_total: usize = hand_query
      .iter()
      .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
      .sum();

    if hand_total < 20 && scores.cpu + hand_total < 100 {
      let new_roll: u32 = rng.range(1..7);

      if new_roll == 1 {
        clear_die(&hand_query, &mut commands);

        state.set(GamePhase::Player);
      } else {
        spawn_die(
          &hand_query,
          &mut commands,
          &assets,
          new_roll as usize,
          Color::Srgba(Srgba::new(0., 0., 1., 1.)),
        );
      }
    } else {
      scores.cpu += hand_total;

      state.set(GamePhase::Player);

      hand_query
        .iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
    }
  }
}

fn display_score(
  scores: Res<Scores>,
  mut egui_context: EguiContexts,
) -> Result {
  egui::Window::new("Total Scores").show(
    egui_context.ctx_mut()?,
    |ui: &mut egui::Ui| {
      ui.label(&format!("Player: {}", scores.player));

      ui.label(&format!("CPU: {}", scores.cpu));
    },
  );

  Ok(())
}

fn player(
  hand_query: Query<(Entity, &Sprite), With<HandDie>>,
  mut commands: Commands,
  mut rng: ResMut<RandomNumberGenerator>,
  assets: Res<GameAssets>,
  mut scores: ResMut<Scores>,
  mut state: ResMut<NextState<GamePhase>>,
  mut egui_context: EguiContexts,
) -> Result {
  egui::Window::new("Play Options").show(
    egui_context.ctx_mut()?,
    |ui: &mut egui::Ui| {
      let hand_score: usize = hand_query
        .iter()
        .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
        .sum();

      ui.label(&format!("Score for this hand: {hand_score}"));

      if ui.button("Roll Dice").clicked() {
        let new_roll: usize = rng.range(1..=6);

        if new_roll == 1 {
          clear_die(&hand_query, &mut commands);

          state.set(GamePhase::Cpu);
        } else {
          spawn_die(&hand_query, &mut commands, &assets, new_roll, Color::WHITE)
        }
      }

      if ui.button("Pass - Keep Hand Score").clicked() {
        let hand_total: usize = hand_query
          .iter()
          .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
          .sum();

        scores.player += hand_total;

        clear_die(&hand_query, &mut commands);

        state.set(GamePhase::Cpu);
      }
    },
  );

  Ok(())
}

fn setup(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
  commands.spawn(Camera2d::default());

  let texture: Handle<Image> = asset_server.load("die-faces.png");

  let layout: TextureAtlasLayout =
    TextureAtlasLayout::from_grid(UVec2::splat(256), 3, 2, None, None);

  let texture_atlas_layout: Handle<TextureAtlasLayout> =
    texture_atlas_layouts.add(layout);

  commands.insert_resource(GameAssets {
    image: texture,
    layout: texture_atlas_layout,
  });

  commands.insert_resource(Scores {
    cpu: 0,
    player: 0,
  });

  commands
    .insert_resource(HandTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
}

fn spawn_die(
  hand_query: &Query<(Entity, &Sprite), With<HandDie>>,
  commands: &mut Commands,
  assets: &GameAssets,
  new_roll: usize,
  color: Color,
) {
  let rolled_die: f32 = hand_query.iter().count() as f32 * 256.;

  let mut sprite: Sprite = Sprite::from_atlas_image(
    assets.image.clone(),
    TextureAtlas {
      layout: assets.layout.clone(),
      index: new_roll - 1,
    },
  );

  sprite.color = color;

  commands.spawn((
    sprite,
    Transform::from_xyz(rolled_die - 400., 60., 1.),
    HandDie,
  ));
}
