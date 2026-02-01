use super::scores::Scores;
use ::bevy::prelude::*;

#[derive(Resource)]
pub struct FinalScore(pub Scores);
