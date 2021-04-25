use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Position;
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSprite {
    pub id: i32,
    pub avatar: String,
    pub animation: HashMap<String, Animation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSheet {
    pub id: i32,
    pub image_path: String,
    pub animation: HashMap<String, Vec<i32>>,
    pub frame_height: i32,
    pub frame_width: i32,
    pub frame_per_row: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSequence {
    pub id: i32,
    pub images: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Animation {
    SpriteSheet(i32, String),
    SpriteSequence(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationEffect {
    pub id: i32,
    pub animation: Animation,
    pub percentage: f32,
    pub position: Position,
}
