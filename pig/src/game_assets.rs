use ::bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
  pub image: Handle<Image>,
  pub layout: Handle<TextureAtlasLayout>,
}
