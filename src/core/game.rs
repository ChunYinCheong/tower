use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    tile_map::TileSheet, Action, AnimationEffect, Area, AreaSystem, Attribute, Character,
    CharacterAction, CharacterCrowdControl, CharacterSprite, Command, Controller, CurrentShop,
    FloorSystem, Item, NovelSystem, Position, Race, ShopSystem, SpriteSequence, SpriteSheet,
    Target, Teleportation, Tile, TileMap, Turn, TurnKind, TurnSystem,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    pub character_id: i32,
    pub extend: i32,
    pub tile_size: i32,
    pub border: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub characters: HashMap<i32, Character>,
    pub character_actions: HashMap<i32, CharacterAction>,
    pub commands: HashMap<i32, Command>,
    pub actions: HashMap<i32, Action>,
    pub items: HashMap<i32, Item>,
    pub tile_maps: HashMap<i32, TileMap>,
    pub teleportations: HashMap<i32, Teleportation>,
    pub tiles: HashMap<i32, Tile>,
    pub tile_sheets: HashMap<i32, TileSheet>,
    pub character_sprites: HashMap<i32, CharacterSprite>,
    pub sprite_sheets: HashMap<i32, SpriteSheet>,
    pub sprite_sequences: HashMap<i32, SpriteSequence>,
    pub sprite_animations: HashMap<i32, AnimationEffect>,
    pub areas: HashMap<i32, Area>,
    pub camera: Camera,
    pub turn_system: TurnSystem,
    pub novel_system: NovelSystem,
    pub shop_system: ShopSystem,
    pub floor_system: FloorSystem,
}

impl Game {
    // Turn System
    pub fn add_turn(&mut self, turn: Turn) {
        // assert!(self.turn_system.turn_queue.is_sorted_by_key(|turn| turn.time));
        let index = self
            .turn_system
            .turn_queue
            .iter()
            .enumerate()
            .find(|(_, t)| t.time > turn.time)
            .map(|(i, _)| i)
            .unwrap_or(self.turn_system.turn_queue.len());
        // log::debug!("{:?} {:?} {:?}", &index, &turn, &self.turn_system.turn_queue);
        self.turn_system.turn_queue.insert(index, turn);
        // self.turn_system.turn_queue.push(turn);
        // self.turn_system.turn_queue.sort_by_key(|t| t.time);
    }

    pub fn set_character_command(&mut self, character_id: i32, command: Command) {
        if let Some(character) = self.characters.get_mut(&character_id) {
            let id = 1 + *self.commands.keys().max().unwrap_or(&0);
            self.commands.insert(id, command);
            character.command_id = Some(id);
        }
    }

    pub fn add_command(&mut self, character_id: i32, action_name: String, target: Target) {
        let action_id = self
            .characters
            .get(&character_id)
            .and_then(|c| {
                c.character_action_ids
                    .iter()
                    .filter_map(|id| self.character_actions.get(id))
                    .filter_map(|ca| self.actions.get(&ca.action_id))
                    .find(|a| a.name == action_name)
                    .map(|a| a.id)
            })
            .unwrap();
        let new_command = Command::new(self, character_id, action_id, target);
        if let Ok(command) = new_command {
            self.set_character_command(character_id, command);
        }
    }

    pub fn add_player_command(&mut self, character_id: i32, command: Command) {
        if self.turn_system.waiting_input {
            self.set_character_command(character_id, command);
            self.turn_system.waiting_input = false;
        }
    }

    // Novel System
    pub fn start_novel(&mut self, id: i32, character_id: i32, target_id: i32) {
        self.turn_system.pause = true;
        NovelSystem::start(self, id, character_id, target_id);
    }

    pub fn novel_run(&mut self) {
        NovelSystem::run(self);
    }

    pub fn select_option(&mut self, i: usize) {
        NovelSystem::select(self, i);
    }

    pub fn open_shop(&mut self, character_id: i32, shop_character_id: i32) {
        self.novel_system.pause = true;
        self.shop_system.current = Some(CurrentShop {
            character_id,
            shop_character_id,
        });
    }

    pub fn character_change_position(&mut self, character_id: i32, x: i32, y: i32, map_id: i32) {
        let position = Position { x, y, map_id };
        self.character_set_position(character_id, position);
    }

    pub fn cancel_select_ability(&mut self) {
        // todo
    }

    pub fn select_ability(&mut self, index: usize) {
        NovelSystem::select_ability(self, index);
    }

    // Shop System
    pub fn close_shop(&mut self) {
        ShopSystem::close_shop(self);
        self.novel_system.pause = false;
        NovelSystem::run(self);
    }

    pub fn buy_item(&mut self, i: usize) {
        ShopSystem::buy_item(self, i);
    }

    // Game
    pub fn update(&mut self, delta: f32) {
        TurnSystem::update(self, delta);
        NovelSystem::update(self, delta);
    }

    pub fn character_set_position(&mut self, character_id: i32, position: Position) {
        let c = self.characters.get_mut(&character_id).unwrap();
        c.position = position;
        if let Some(_) = self.teleportation_at_position(&position) {
            FloorSystem::next(self);
        }
        AreaSystem::character_position_changed(self, character_id);
    }
    pub fn character_at_position(&self, position: &Position) -> Option<i32> {
        self.characters
            .iter()
            .find(|(_, c)| c.position == *position && !c.hidden)
            .map(|(id, _)| *id)
    }

    pub fn characters_in_range(&self, position: &Position, range: i32) -> Vec<i32> {
        self.characters
            .iter()
            .filter(|(_, c)| Position::distance(position, &c.position) <= range)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn characters_in_square(&self, position: &Position, range: i32) -> Vec<i32> {
        self.characters
            .iter()
            .filter(|(_, c)| {
                position.map_id == c.position.map_id
                    && (position.x - c.position.x).abs() <= range
                    && (position.y - c.position.y).abs() <= range
            })
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn teleportation_at_position(&self, position: &Position) -> Option<i32> {
        self.teleportations
            .iter()
            .find(|(_, c)| c.position == *position)
            .map(|(id, _)| *id)
    }

    pub fn new_player_character(&mut self) {
        let mut character_action_ids = Vec::new();
        for action_id in 1..=4 {
            let id = 1 + *self.character_actions.keys().max().unwrap_or(&0);
            let v = CharacterAction::new(action_id);
            self.character_actions.insert(id, v);
            character_action_ids.push(id);
        }

        let id = 1 + *self.characters.keys().max().unwrap_or(&0);
        let c = Character {
            id,
            crowd_controls: CharacterCrowdControl {
                stun: 0,
                charm: 0,
                shock: 0,
                shocked: false,
                poison: 0,
                sleep: 0,
                root: 0,
                silent: 0,
            },
            position: Position {
                map_id: 0,
                x: 5,
                y: 8,
            },
            controller: Controller::Player,
            items: Default::default(),
            character_action_ids,
            race: Race::Human,
            character_sprite_id: 1,
            offset_x: 0.0,
            offset_y: 0.0,
            defeated: false,
            dead: false,
            experience: 8945,
            level: 20,
            hp: Attribute {
                base: 20,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            mp: Attribute {
                base: 20,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            attack: Attribute {
                base: 10,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            defence: Attribute {
                base: 5,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            sanity: Attribute {
                base: 10,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            hidden: false,
            command_id: None,
            talk_id: 1,
            shop: None,
            gold: 0,
            wood: 0,
        };
        self.characters.insert(id, c);
        self.add_turn(Turn {
            time: 0,
            kind: TurnKind::Character { character_id: id },
        });
    }

    pub fn new_enemy(&mut self) -> i32 {
        let mut character_action_ids = Vec::new();
        for action_id in 1..=4 {
            let id = 1 + *self.character_actions.keys().max().unwrap_or(&0);
            let v = CharacterAction::new(action_id);
            self.character_actions.insert(id, v);
            character_action_ids.push(id);
        }

        let id = 1 + *self.characters.keys().max().unwrap_or(&0);
        let c = Character {
            id,
            crowd_controls: CharacterCrowdControl {
                stun: 0,
                charm: 0,
                shock: 0,
                shocked: false,
                poison: 0,
                sleep: 0,
                root: 0,
                silent: 0,
            },
            position: Position {
                map_id: 0,
                x: 5,
                y: 8,
            },
            controller: Controller::Enemy,
            items: Default::default(),
            character_action_ids,
            race: Race::Demon,
            character_sprite_id: 2,
            offset_x: 0.0,
            offset_y: 0.0,
            defeated: false,
            dead: false,
            experience: 0,
            level: 1,
            hp: Attribute {
                base: 10,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            mp: Attribute {
                base: 10,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            attack: Attribute {
                base: 10,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            defence: Attribute {
                base: 5,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            sanity: Attribute {
                base: 10,
                drain: 0,
                damage: 0,
                modifier: 0,
            },
            hidden: false,
            command_id: None,
            talk_id: 1,
            shop: None,
            gold: 0,
            wood: 0,
        };
        self.characters.insert(id, c);
        self.add_turn(Turn {
            time: 0,
            kind: TurnKind::Character { character_id: id },
        });
        id
    }
}
