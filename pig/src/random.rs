use ::bevy::prelude::*;
use ::my_lib::RandomNumberGenerator;

#[derive(Resource)]
pub struct Random(pub RandomNumberGenerator);
