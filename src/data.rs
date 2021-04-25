use std::collections::HashMap;

use ggez::{graphics, Context, GameResult};
use serde::{Deserialize, Serialize};
use tower::core::{Game, Target};

#[derive(Debug)]
pub struct Data {
    pub game: Game,
    pub target_scene: TargetSceneData,
    pub action_scene: ActionSceneData,
    pub image_caches: ImageCache,
}

impl Data {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            target_scene: Default::default(),
            action_scene: Default::default(),
            image_caches: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ImageCache {
    pub resources: HashMap<String, graphics::Image>,
}

impl ImageCache {
    pub fn get(&mut self, ctx: &mut Context, key: &str) -> GameResult<&graphics::Image> {
        if !self.resources.contains_key(key) {
            let image = graphics::Image::new(ctx, key)?;
            self.resources.insert(String::from(key), image);
        }
        Ok(self.resources.get(key).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TargetSceneData {
    pub finish: bool,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ActionSceneData {
    pub action_id: Option<i32>,
}
