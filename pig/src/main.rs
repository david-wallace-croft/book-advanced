use self::final_score::FinalScore;
use self::game_assets::GameAssets;
use self::game_element::GameElement;
use self::game_phase::GamePhase;
use self::hand_die::HandDie;
use self::hand_timer::HandTimer;
use self::scores::Scores;
use ::bevy::prelude::*;
use ::bevy::window::WindowResolution;
use ::bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use ::my_lib::game_state_plugin::GameStatePlugin;
use ::my_lib::random::RandomNumberGenerator;
use ::my_lib::random_plugin::RandomPlugin;
use ::my_lib::{add_phase, cleanup};

mod final_score;
mod game_assets;
mod game_element;
mod game_phase;
mod hand_die;
mod hand_timer;
mod random;
mod scores;

fn main() {
  let mut app: App = App::new();

  add_phase!(app, GamePhase, GamePhase::Start,
      start => [ setup ],
      run => [ start_game ],
      exit => [ ]
  );

  add_phase!(app, GamePhase, GamePhase::Player,
      start => [ ],
      run => [ check_game_over ],
      exit => [ ]
  );

  add_phase!(app, GamePhase, GamePhase::Cpu,
      start => [ ],
      run => [ cpu, check_game_over, display_score ],
      exit => [ ]
  );

  add_phase!(app, GamePhase, GamePhase::End,
      start => [ ],
      run => [ end_game ],
      exit => [ cleanup::<GameElement> ]
  );

  add_phase!(app, GamePhase, GamePhase::GameOver,
      start => [ ],
      run => [ display_final_score ],
      exit => [ ]
  );

  let resolution: WindowResolution = WindowResolution::new(1024, 768);

  let primary_window: Window = Window {
    title: "Pig".into(),
    resolution,
    ..default()
  };

  let window_plugin = WindowPlugin {
    primary_window: Some(primary_window),
    ..default()
  };

  let game_state_plugin: GameStatePlugin<GamePhase> = GameStatePlugin {
    menu_state: GamePhase::MainMenu,
    game_start_state: GamePhase::Start,
    game_end_state: GamePhase::GameOver,
  };

  let egui_plugin: EguiPlugin = EguiPlugin::default();

  app
    .add_plugins(DefaultPlugins.set(window_plugin))
    .add_plugins(game_state_plugin)
    .add_plugins(egui_plugin)
    .add_plugins(RandomPlugin)
    // .add_systems(Startup, setup)
    .add_systems(EguiPrimaryContextPass, display_score)
    .init_state::<GamePhase>()
    // .add_systems(EguiPrimaryContextPass, player)
    .add_systems(
      EguiPrimaryContextPass,
      player.run_if(in_state(GamePhase::Player)),
    )
    // .add_systems(Update, cpu.run_if(in_state(GamePhase::Cpu)))
    .run();
}

fn check_game_over(
  scores: Res<Scores>,
  mut state: ResMut<NextState<GamePhase>>,
) {
  if scores.cpu >= 100 || scores.player >= 100 {
    state.set(GamePhase::End);
  }
}

fn clear_die(
  hand_query: &Query<(Entity, &Sprite), With<HandDie>>,
  commands: &mut Commands,
) {
  hand_query
    .iter()
    .for_each(|(entity, _)| commands.entity(entity).despawn());
}

#[expect(clippy::too_many_arguments)]
fn cpu(
  hand_query: Query<(Entity, &Sprite), With<HandDie>>,
  mut state: ResMut<NextState<GamePhase>>,
  mut scores: ResMut<Scores>,
  rng: Res<RandomNumberGenerator>,
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

fn display_final_score(
  mut egui_contexts: EguiContexts,
  scores: Res<FinalScore>,
) {
  let Ok(egui_context) = egui_contexts.ctx_mut() else {
    return;
  };

  egui::Window::new("Total Scores").show(egui_context, |ui| {
    ui.label(format!("Player: {}", scores.0.player));

    ui.label(format!("CPU: {}", scores.0.cpu));

    if scores.0.player < scores.0.cpu {
      ui.label("CPU is the winner!");
    } else {
      ui.label("Player is the winner!");
    }
  });
}

fn display_score(
  scores: Res<Scores>,
  mut egui_contexts: EguiContexts,
) -> Result {
  let Ok(egui_context) = egui_contexts.ctx_mut() else {
    return Ok(());
  };

  egui::Window::new("Total Scores").show(egui_context, |ui: &mut egui::Ui| {
    ui.label(format!("Player: {}", scores.player));

    ui.label(format!("CPU: {}", scores.cpu));
  });

  Ok(())
}

fn end_game(
  mut commands: Commands,
  scores: Res<Scores>,
  mut state: ResMut<NextState<GamePhase>>,
) {
  commands.insert_resource(FinalScore(*scores));

  state.set(GamePhase::GameOver);
}

fn player(
  hand_query: Query<(Entity, &Sprite), With<HandDie>>,
  mut commands: Commands,
  rng: Res<RandomNumberGenerator>,
  assets: Res<GameAssets>,
  mut scores: ResMut<Scores>,
  mut state: ResMut<NextState<GamePhase>>,
  mut egui_contexts: EguiContexts,
) -> Result {
  let Ok(egui_context) = egui_contexts.ctx_mut() else {
    return Ok(());
  };

  egui::Window::new("Play Options").show(egui_context, |ui: &mut egui::Ui| {
    let hand_score: usize = hand_query
      .iter()
      .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
      .sum();

    ui.label(format!("Score for this hand: {hand_score}"));

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
  });

  Ok(())
}

fn setup(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
  commands.spawn(Camera2d).insert(GameElement);

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
    GameElement,
  ));
}

fn start_game(mut state: ResMut<NextState<GamePhase>>) {
  state.set(GamePhase::Player);
}
