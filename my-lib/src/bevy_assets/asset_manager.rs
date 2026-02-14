use super::asset_store::AssetStore;
use super::asset_type::AssetType;
use ::bevy::platform::collections::HashMap;
use ::bevy::prelude::*;
// use ::std::env;
// use ::std::path::PathBuf;

#[derive(Clone, Resource)]
pub struct AssetManager {
  asset_list: Vec<(String, String, AssetType)>,
}

impl AssetManager {
  pub fn new() -> Self {
    Self {
      asset_list: Vec::new(),
    }
  }

  pub fn add_image<S: ToString>(
    mut self,
    tag: S,
    filename: S,
  ) -> ::anyhow::Result<Self> {
    let filename: String = filename.to_string();

    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //   let current_directory: PathBuf = env::current_dir()?;

    //   // let assets: PathBuf = current_directory.join("assets");

    //   let new_image: PathBuf = current_directory.join(&filename);

    //   if !new_image.exists() {
    //     return Err(::anyhow::Error::msg(format!(
    //       "{filename} not found in assets directory"
    //     )));
    //   }
    // }

    self
      .asset_list
      .push((tag.to_string(), filename, AssetType::Image));

    Ok(self)
  }
}

impl Plugin for AssetManager {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.insert_resource(self.clone());

    app.add_systems(Startup, setup);
  }
}

fn setup(
  asset_resource: Res<AssetManager>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  let mut assets: AssetStore = AssetStore {
    asset_index: HashMap::new(),
  };

  asset_resource
    .asset_list
    .iter()
    .for_each(|(tag, filename, asset_type)| match asset_type {
      _ => {
        assets
          .asset_index
          .insert(tag.clone(), asset_server.load_untyped(filename));
      },
    });

  commands.remove_resource::<AssetManager>();

  commands.insert_resource(assets);
}
