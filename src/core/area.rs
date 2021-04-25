use super::Game;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    pub map_id: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    StartBossFight,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaSystem;

impl AreaSystem {
    pub fn character_position_changed(game: &mut Game, character_id: i32) {
        if let Some(position) = game.characters.get(&character_id).map(|c| c.position) {
            for (_, area) in game.areas.iter().filter(|(_, a)| {
                a.map_id == position.map_id
                    && a.x <= position.x
                    && position.x < a.x + a.w
                    && a.y <= position.y
                    && position.y < a.y + a.h
            }) {
                for event in &area.events {
                    match event {
                        Event::StartBossFight => {
                            // ....
                            for (_, c) in
                                game.characters
                                    .iter_mut()
                                    .filter(|(_, c)| match c.controller {
                                        super::Controller::Boss { waiting: _ } => true,
                                        _ => false,
                                    })
                            {
                                match c.controller {
                                    super::Controller::Boss { ref mut waiting } => *waiting = false,
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
