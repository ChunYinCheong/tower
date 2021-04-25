use super::{command::CommandError, Attribute, Command, Game, Position, Target, TurnKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub crowd_controls: CharacterCrowdControl,
    pub position: Position,
    pub controller: Controller,
    pub items: Vec<CharacterItem>,
    pub character_action_ids: Vec<i32>,
    pub race: Race,
    pub character_sprite_id: i32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub defeated: bool,
    pub dead: bool,
    pub experience: i32,
    pub level: i32,
    pub hp: Attribute,
    pub mp: Attribute,
    pub attack: Attribute,
    pub defence: Attribute,
    pub sanity: Attribute,
    pub hidden: bool,
    pub command_id: Option<i32>,
    pub talk_id: i32,
    pub shop: Option<Shop>,
    pub gold: i32,
    pub wood: i32,
}

impl Character {
    pub fn move_command(
        game: &mut Game,
        character_id: i32,
        x: i32,
        y: i32,
    ) -> Result<Command, CommandError> {
        if let Some(c) = game.characters.get(&character_id) {
            let action_id = game
                .characters
                .get(&character_id)
                .and_then(|c| {
                    c.character_action_ids
                        .iter()
                        .filter_map(|id| game.character_actions.get(id))
                        .filter_map(|ca| game.actions.get(&ca.action_id))
                        .find(|a| a.name == String::from("Move"))
                        .map(|a| a.id)
                })
                .unwrap();
            let target = Target::Position(Position {
                map_id: c.position.map_id,
                x: c.position.x + x,
                y: c.position.y + y,
            });
            Command::new(game, character_id, action_id, target)
        } else {
            Err(CommandError::Target)
        }
    }

    pub fn gain_exp(&mut self, exp: i32) {
        self.set_experience(self.experience + exp);
    }
    pub fn set_experience(&mut self, exp: i32) {
        self.experience = exp;
        // exp = 100 * level ^ 1.5
        // let level = ((self.experience as f32 / 100.0).ln() / 1.5).exp().floor() as i32;
        let level = match self.experience {
            i32::MIN..=4 => 1,
            5..=14 => 2,
            15..=29 => 3,
            30..=49 => 4,
            50..=i32::MAX => 5,
        };
        if level != self.level {
            self.level = level;
            self.hp.base = 100 + level * 10;
            self.mp.base = 100 + level * 10;
        }
    }

    pub fn kill_experience(&self) -> i32 {
        // (100.0 * (1.05 as f32).powi(self.level)) as i32
        1
    }

    pub fn take_damage(game: &mut Game, damage: i32, target_id: i32, source_id: i32) {
        let target = game.characters.get_mut(&target_id).unwrap();
        target.hp.damage += damage;
        if !target.dead && target.hp.current() <= -1000 {
            target.dead = true;
            target.hidden = true;
        }
        if !target.defeated && target.hp.current() <= 0 {
            target.defeated = true;
            let exp = target.kill_experience();
            target.hidden = true;

            let character = game.characters.get_mut(&source_id).unwrap();
            character.gain_exp(exp);

            game.turn_system.turn_queue.retain(|t| match &t.kind {
                TurnKind::Character { character_id } => *character_id != target_id,
                TurnKind::Respawn { character_id: _ } => true,
            });
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Race {
    Human,
    Elf,
    Orc,
    Demon,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Controller {
    Player,
    Enemy,
    NPC,
    Boss { waiting: bool },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCrowdControl {
    pub stun: i32,
    pub charm: i32,
    pub shock: i32,
    pub shocked: bool,
    pub poison: i32,
    pub sleep: i32,
    pub root: i32,
    pub silent: i32,
}

impl CharacterCrowdControl {
    pub fn turn_end(&mut self) {
        self.stun = (self.stun - 1).max(0);
        self.charm = (self.charm - 1).max(0);
        self.shock = (self.shock - 1).max(0);
        self.poison = (self.poison - 1).max(0);
        self.sleep = (self.sleep - 1).max(0);
        self.root = (self.root - 1).max(0);
        self.silent = (self.silent - 1).max(0);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CrowdControlKind {
    // Cannot do anything
    Stun,
    // Move to enemy
    Charm,
    // Stop action every 2 turn
    Shock,
    Poison,
    Sleep,
    // Not moveable
    Root,
    // Disable some abilities
    Silent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAction {
    pub action_id: i32,
    pub cooldown: i32,
    pub exp: i32,
}
impl CharacterAction {
    pub fn new(action_id: i32) -> Self {
        Self {
            action_id,
            cooldown: 0,
            exp: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterItem {
    pub item_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_kind: ItemKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemKind {
    Potion,
    ActionBook(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    pub id: i32,
    pub items: Vec<ShopItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopItem {
    pub item_id: i32,
    pub stock: Option<i32>,
}
