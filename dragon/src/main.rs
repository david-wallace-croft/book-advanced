use self::game_phase::GamePhase;
use self::game_state_plugin::GameStatePlugin;
use ::bevy::prelude::*;
use ::bevy::window::WindowResolution;
use ::my_lib::random_plugin::RandomPlugin;

mod game_phase;
mod game_state_plugin;

fn main() {
  let resolution: WindowResolution = WindowResolution::new(1024, 768);

  let primary_window: Option<Window> = Some(Window {
    resolution,
    title: "Flappy Drag - Bevy Edition".to_string(),
    ..default()
  });

  let window_plugin: WindowPlugin = WindowPlugin {
    primary_window,
    ..default()
  };

  App::new()
    .add_plugins(DefaultPlugins.set(window_plugin))
    .add_plugins(RandomPlugin)
    .add_plugins(GameStatePlugin::<GamePhase> {
      game_end_state: GamePhase::GameOver,
      game_start_state: GamePhase::Flapping,
      menu_state: GamePhase::MainMenu,
    });
}
