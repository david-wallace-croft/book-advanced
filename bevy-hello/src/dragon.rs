use ::bevy::prelude::*;

#[derive(Component)]
pub struct Dragon;

impl Dragon {
  pub fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dragon_query: Query<&mut Transform, With<Dragon>>,
  ) {
    let mut x: f32 = 0.;

    let mut y: f32 = 0.;

    if keyboard.pressed(KeyCode::ArrowLeft) {
      x -= 1.;
    }

    if keyboard.pressed(KeyCode::ArrowRight) {
      x += 1.;
    }

    if keyboard.pressed(KeyCode::ArrowDown) {
      y -= 1.;
    }

    if keyboard.pressed(KeyCode::ArrowUp) {
      y += 1.;
    }

    dragon_query.iter_mut().for_each(|mut transform| {
      transform.translation += Vec3 {
        x,
        y,
        z: 0.,
      };
    })
  }
}
