#![expect(dead_code)]

use ::bevy::prelude::*;
use ::my_lib::random_locking::RandomNumberGenerator;

#[derive(Resource)]
pub struct Random(pub RandomNumberGenerator);
