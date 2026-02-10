use ::bevy::asset::LoadedUntypedAsset;
use ::bevy::prelude::*;

#[expect(unused)]
#[derive(Resource)]
pub(crate) struct AssetsToLoad(Vec<Handle<LoadedUntypedAsset>>);
