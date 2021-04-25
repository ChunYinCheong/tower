use super::{Controller, Game, Position};
use serde::{Deserialize, Serialize};

pub const TEMP_MAP_ID: i32 = -1;
pub const BOSS_MAP_ID: i32 = -2;
pub const NORMAL_MAP_ID: i32 = -3;
pub const SKILL_VENDOR_ID: i32 = -1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub character_id: i32,
    pub floor: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloorSystem {
    pub pause: bool,
    pub current: Option<Current>,
}

impl FloorSystem {
    pub fn start(game: &mut Game, character_id: i32) {
        let c = Current {
            character_id,
            floor: 0,
        };
        game.floor_system.current = Some(c);
    }

    pub fn next(game: &mut Game) {
        if let Some(current) = &mut game.floor_system.current {
            let floor = current.floor + 1;
            current.floor = floor;

            if let Some(character) = game.characters.get_mut(&current.character_id) {
                character.experience += 1;
                character.gold += 1;
                character.wood += 1;
                for id in &character.character_action_ids {
                    if let Some(ca) = game.character_actions.get_mut(id) {
                        ca.exp += 1;
                    }
                }
                if floor % 5 == 0 || (floor - 1) % 5 == 0 {
                    // Recover when enter/leave Boss room
                    character.hp.damage = 0;
                    character.mp.damage = 0;
                }
                character.position = if floor % 5 == 0 {
                    // Boss
                    Position {
                        map_id: BOSS_MAP_ID,
                        x: 5,
                        y: 10,
                    }
                } else {
                    // Normal
                    Position {
                        map_id: NORMAL_MAP_ID,
                        x: 0,
                        y: 2,
                    }
                };
            }
            for (_, c) in game
                .characters
                .iter_mut()
                .filter(|(_, c)| match c.controller {
                    Controller::Player => false,
                    _ => true,
                })
                .filter(|(_, c)| {
                    c.position.map_id == BOSS_MAP_ID || c.position.map_id == NORMAL_MAP_ID
                })
            {
                c.position.map_id = TEMP_MAP_ID;
            }
            match floor {
                1..=4
                | 6..=9
                | 11..=14
                | 16..=19
                | 21..=24
                | 26..=29
                | 31..=34
                | 36..=39
                | 41..=44
                | 46..=49 => {
                    // Set up enemy
                    let id = game.new_enemy();
                    if let Some(enemy) = game.characters.get_mut(&id) {
                        enemy.position = Position {
                            map_id: NORMAL_MAP_ID,
                            x: 9,
                            y: 2,
                        };
                        enemy.hp.base = floor;
                    }
                }
                0 | 5 | 10 | 15 | 20 | 25 | 30 | 35 | 40 | 45 | 50 => {
                    let character_id = current.character_id;
                    // Vendor
                    if let Some(c) = game.characters.get_mut(&SKILL_VENDOR_ID) {
                        c.position.map_id = BOSS_MAP_ID;
                        c.position.x = 4;
                        c.position.y = 10;
                    }
                    // Boss
                    let id = game.new_enemy();
                    if let Some(enemy) = game.characters.get_mut(&id) {
                        enemy.position = Position {
                            map_id: BOSS_MAP_ID,
                            y: 5,
                            x: 5,
                        };
                        enemy.hp.base = 100 + floor;
                        enemy.controller = super::Controller::Boss { waiting: true };
                    }
                    // Talk to player
                    let talk_id = 0;
                    game.start_novel(talk_id, character_id, id);
                }
                _ => {
                    log::error!("Floor out of range");
                }
            }
        } else {
            log::error!("No current in FloorSystem");
        }
    }
}
