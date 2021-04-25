use super::{CharacterAction, Command, CommandState, Game, ItemKind, Position, Target};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct NovelSystem {
    pub pause: bool,
    pub scripts: HashMap<i32, Script>,
    pub current: Option<CurrentScript>,
}

impl NovelSystem {
    pub fn update(game: &mut Game, delta: f32) {
        if game.novel_system.pause {
            return;
        }
        if let Some(current) = &mut game.novel_system.current {
            if let Some(command_id) = current.command_id {
                let command = game.commands.get(&command_id).unwrap();
                match command.state {
                    CommandState::New => {
                        Command::start(game, command_id);
                    }
                    CommandState::Started => {
                        let command = game.commands.get_mut(&command_id).unwrap();
                        // let wait_animation = character_map == camera_map;
                        let duration = game.actions.get(&command.action_id).unwrap().duration;
                        command.elapsed_time = command.elapsed_time + delta;
                        let animation_end = command.elapsed_time >= duration;

                        if !animation_end {
                            Command::update(game, command_id);
                        } else {
                            Command::end(game, command_id);
                        }
                    }
                    CommandState::Ended => {
                        game.commands.remove(&command_id);
                        current.command_id = None;
                        NovelSystem::run(game);
                    }
                }
            }
        }
    }
    pub fn start(game: &mut Game, id: i32, character_id: i32, target_id: i32) {
        game.novel_system.current = Some(CurrentScript {
            id,
            character_id,
            target_id,
            p: vec![0],
            history: Default::default(),
            select: None,
            select_ability_character_action_index: None,
            command_id: None,
            waiting_select_ability: false,
            background: None,
        });
        NovelSystem::run(game);
    }

    pub fn run(game: &mut Game) {
        // Clear command
        if let Some(command_id) = game
            .novel_system
            .current
            .as_ref()
            .and_then(|c| c.command_id)
        {
            loop {
                let command = game.commands.get(&command_id).unwrap();
                match command.state {
                    CommandState::New => {
                        Command::start(game, command_id);
                    }
                    CommandState::Started => {
                        Command::end(game, command_id);
                    }
                    CommandState::Ended => {
                        game.commands.remove(&command_id);
                        if let Some(current) = &mut game.novel_system.current {
                            current.command_id = None;
                        }
                        break;
                    }
                }
            }
        }
        if let Some(current) = &mut game.novel_system.current {
            // Clear select
            current.select = None;
            // Run
            if let Some(script) = game.novel_system.scripts.get(&current.id) {
                if current.p.is_empty() {
                    game.novel_system.current = None;
                    game.turn_system.pause = false;
                } else {
                    let mut c = script.scripts.get(current.p[0]);
                    let mut i = 1;
                    while i < current.p.len() {
                        if let Some(s) = c {
                            match s {
                                ScriptKind::Text(_) => panic!(),
                                ScriptKind::If(_, v) => {
                                    c = v.get(current.p[i]);
                                }
                                ScriptKind::IfElse(_, _, _) => panic!(),
                                ScriptKind::Select(v) => {
                                    if i + 1 < current.p.len() {
                                        c = v
                                            .get(current.p[i])
                                            .and_then(|(_, v)| v.get(current.p[i + 1]));
                                    } else {
                                        c = None;
                                    }
                                    i += 1;
                                }
                                ScriptKind::Shop => panic!(),
                                ScriptKind::Surrender => panic!(),
                                ScriptKind::Restart => panic!(),
                                ScriptKind::CharacterMove(_, _, _) => panic!(),
                                ScriptKind::CharacterChangePosition(_, _, _, _) => panic!(),
                                ScriptKind::SelectAbility(v) => {
                                    c = v.get(current.p[i]);
                                }
                                ScriptKind::ForgetSkill => panic!(),
                                ScriptKind::Background(_) => panic!(),
                                ScriptKind::PracticeSkill => panic!(),
                            }
                        }
                        i += 1;
                    }

                    if let Some(s) = c {
                        match s {
                            ScriptKind::Text(s) => {
                                current.history.push(s.clone());
                                if let Some(i) = current.p.last_mut() {
                                    *i += 1;
                                }
                            }
                            ScriptKind::If(condition, _) => {
                                if Condition::is_met(game, condition) {
                                    if let Some(current) = &mut game.novel_system.current {
                                        current.p.push(0);
                                    }
                                }
                                NovelSystem::run(game);
                            }
                            ScriptKind::IfElse(_, _, _) => {}
                            ScriptKind::Select(v) => {
                                current.select =
                                    Some(v.iter().map(|(s, _)| String::from(s)).collect());
                            }
                            ScriptKind::Shop => {
                                if let Some(i) = current.p.last_mut() {
                                    *i += 1;
                                }
                                let character_id = current.character_id;
                                let shop_character_id = current.target_id;
                                game.open_shop(character_id, shop_character_id);
                            }
                            ScriptKind::Surrender => {}
                            ScriptKind::Restart => {
                                current.p = vec![0];
                                NovelSystem::run(game);
                            }
                            ScriptKind::CharacterMove(c, x, y) => {
                                if let Some(i) = current.p.last_mut() {
                                    *i += 1;
                                }
                                let character_id = match c {
                                    ScriptCharacter::Initiator => current.character_id,
                                    ScriptCharacter::Target => current.target_id,
                                };
                                // game.character_move(character_id, *x, *y);
                                let action_id = game
                                    .characters
                                    .get(&character_id)
                                    .and_then(|c| {
                                        c.character_action_ids
                                            .iter()
                                            .filter_map(|id| game.character_actions.get(id))
                                            .find(|ca| {
                                                game.actions.get(&ca.action_id).map(|a| &a.name[..])
                                                    == Some("Move")
                                            })
                                            .map(|a| a.action_id)
                                    })
                                    .unwrap();
                                let target = Target::Position(Position {
                                    x: *x,
                                    y: *y,
                                    map_id: game
                                        .characters
                                        .get(&character_id)
                                        .map(|c| c.position.map_id)
                                        .unwrap(),
                                });
                                let command =
                                    Command::new(game, character_id, action_id, target).unwrap();
                                if let Some(current) = &mut game.novel_system.current {
                                    let id = 1 + *game.commands.keys().max().unwrap_or(&0);
                                    game.commands.insert(id, command);
                                    current.command_id = Some(id);
                                }
                            }
                            ScriptKind::CharacterChangePosition(c, x, y, map_id) => {
                                if let Some(i) = current.p.last_mut() {
                                    *i += 1;
                                }
                                let character_id = match c {
                                    ScriptCharacter::Initiator => current.character_id,
                                    ScriptCharacter::Target => current.target_id,
                                };
                                game.character_change_position(character_id, *x, *y, *map_id);
                                NovelSystem::run(game);
                            }
                            ScriptKind::SelectAbility(_) => {
                                current.waiting_select_ability = true;
                            }
                            ScriptKind::ForgetSkill => {
                                if let Some(index) = current.select_ability_character_action_index {
                                    if let Some(character) =
                                        game.characters.get_mut(&current.character_id)
                                    {
                                        character.character_action_ids.remove(index);
                                    }
                                }
                                if let Some(i) = current.p.last_mut() {
                                    *i += 1;
                                }
                                NovelSystem::run(game);
                            }
                            ScriptKind::Background(background) => {
                                current.background = Some(Background::clone(background));
                                if let Some(i) = current.p.last_mut() {
                                    *i += 1;
                                }
                                NovelSystem::run(game);
                            }
                            ScriptKind::PracticeSkill => {
                                if let Some(i) = current.p.last_mut() {
                                    *i += 1;
                                }

                                let character_id = current.character_id;
                                let action_ids = game.characters.get(&character_id).map(|c| {
                                    c.items
                                        .iter()
                                        .map(|ci| ci.item_id)
                                        .map(|id| game.items.get(&id))
                                        .flatten()
                                        .filter_map(|item| match item.item_kind {
                                            ItemKind::ActionBook(id) => {
                                                if c.character_action_ids
                                                    .iter()
                                                    .filter_map(|id| game.character_actions.get(id))
                                                    .any(|a| a.action_id == id)
                                                {
                                                    None
                                                } else {
                                                    Some(id)
                                                }
                                            }
                                            _ => None,
                                        })
                                        .collect::<HashSet<_>>()
                                        .into_iter()
                                        .collect::<Vec<_>>()
                                });
                                if let Some(ids) = action_ids {
                                    for action_id in ids {
                                        let id =
                                            1 + *game.character_actions.keys().max().unwrap_or(&0);
                                        let v = CharacterAction::new(action_id);
                                        game.character_actions.insert(id, v);
                                        if let Some(c) = game.characters.get_mut(&character_id) {
                                            c.character_action_ids.push(id);
                                        }
                                    }
                                    if let Some(c) = game.characters.get_mut(&character_id) {
                                        let items = &game.items;
                                        c.items.retain(|ci| {
                                            items
                                                .get(&ci.item_id)
                                                .map(|item| match item.item_kind {
                                                    ItemKind::ActionBook(_) => false,
                                                    _ => true,
                                                })
                                                .unwrap_or(true)
                                        });
                                    }
                                }

                                NovelSystem::run(game);
                            }
                        }
                    } else {
                        current.p.pop();
                        if let Some(i) = current.p.last_mut() {
                            *i += 1;
                        }
                        NovelSystem::run(game);
                    }
                }
            }
        }
    }

    pub fn next(game: &mut Game) {
        if let Some(current) = game.novel_system.current.as_mut() {
            if let Some(i) = current.p.last_mut() {
                *i += 1;
            }
            NovelSystem::run(game);
        }
    }

    pub fn select(game: &mut Game, i: usize) {
        if let Some(current) = &mut game.novel_system.current {
            current.p.push(i);
            current.p.push(0);
            NovelSystem::run(game);
        }
    }

    pub fn select_ability(game: &mut Game, index: usize) {
        if let Some(current) = &mut game.novel_system.current {
            current.waiting_select_ability = false;
            current.select_ability_character_action_index = Some(index);
            current.p.push(0);
            NovelSystem::run(game);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentScript {
    pub id: i32,
    pub character_id: i32,
    pub target_id: i32,
    pub p: Vec<usize>,
    pub history: Vec<String>,
    pub select: Option<Vec<String>>,
    pub select_ability_character_action_index: Option<usize>,
    pub waiting_select_ability: bool,
    // pub animation: Option<ScriptAnimation>,
    pub command_id: Option<i32>,
    pub background: Option<Background>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptAnimation {
    pub delta: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    pub id: i32,
    pub scripts: Vec<ScriptKind>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScriptKind {
    Text(String),
    Background(Background),
    If(Condition, Vec<ScriptKind>),
    IfElse(Condition, Vec<ScriptKind>, Vec<ScriptKind>),
    Select(Vec<(String, Vec<ScriptKind>)>),
    Restart,
    Shop,
    Surrender,
    CharacterMove(ScriptCharacter, i32, i32),
    CharacterChangePosition(ScriptCharacter, i32, i32, i32),
    SelectAbility(Vec<ScriptKind>),
    ForgetSkill,
    PracticeSkill,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Condition {}

impl Condition {
    pub fn is_met(game: &Game, condition: &Condition) -> bool {
        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScriptCharacter {
    Initiator,
    Target,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Background {
    None,
    Color(u8, u8, u8, u8),
    Image(String),
}
