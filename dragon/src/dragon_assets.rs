use ::bevy::prelude::*;

#[derive(Resource)]
pub struct DragonAssets {
  pub dragon: Handle<Image>,
  pub wall: Handle<Image>,
}
