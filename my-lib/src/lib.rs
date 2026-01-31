use ::bevy::prelude::*;

#[cfg(not(feature = "locking"))]
pub mod random;

#[cfg(feature = "locking")]
pub mod random_locking;

pub mod game_menus;
pub mod game_state_plugin;
pub mod menu_assets;
pub mod menu_element;
pub mod menu_resource;
pub mod random_plugin;

#[cfg(feature = "locking")]
pub use random_locking as random;

#[macro_export]
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
