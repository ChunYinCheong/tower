use super::{Character, Command, Controller, Game, Position, Race, Target};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnSystem {
    pub turn_queue: Vec<Turn>,
    pub current_turn: Option<Turn>,
    pub waiting_input: bool,
    pub state: State,
    pub pause: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Turn {
    pub time: i32,
    pub kind: TurnKind,
}

impl TurnSystem {
    pub fn update(game: &mut Game, delta: f32) {
        loop {
            if game.turn_system.pause {
                return;
            }
            let (out, state) = match game.turn_system.state {
                State::Empty => TurnSystem::empty(game),
                State::StartTurn => TurnSystem::start_turn(game),
                State::StartCommand => TurnSystem::start_command(game),
                State::UpdateCommand => TurnSystem::update_command(game, delta),
                State::EndCommand => TurnSystem::end_command(game),
                State::EndTurn => TurnSystem::end_turn(game),
            };
            game.turn_system.state = state;
            if out {
                return;
            }
        }
    }

    pub fn empty(game: &mut Game) -> (bool, State) {
        if !game.turn_system.turn_queue.is_empty() {
            let mut t = game.turn_system.turn_queue.remove(0);
            if t.time > 0 {
                for turn in &mut game.turn_system.turn_queue {
                    turn.time -= t.time;
                }
            }
            t.time = 0;
            game.turn_system.current_turn = Some(t);
            return (false, State::StartTurn);
        } else {
            return (true, State::Empty);
        }
    }

    pub fn start_turn(game: &mut Game) -> (bool, State) {
        if let Some(t) = &game.turn_system.current_turn {
            match t.kind {
                TurnKind::Character { character_id } => {
                    let c = game.characters.get_mut(&character_id).unwrap();

                    if c.dead || c.defeated {
                        // Skip turn
                        return (false, State::EndTurn);
                    }
                    if c.crowd_controls.sleep > 0 || c.crowd_controls.stun > 0 {
                        return (false, State::EndTurn);
                    }
                    if c.crowd_controls.shock > 0 {
                        c.crowd_controls.shocked = !c.crowd_controls.shocked;
                        if c.crowd_controls.shocked {
                            return (false, State::EndTurn);
                        }
                    }
                    if c.crowd_controls.charm > 0 {
                        return (false, State::EndTurn);
                        // Move to enemy
                        // character move...
                        // return (false, State::StartCommand);
                    }

                    let c = game.characters.get(&character_id).unwrap();
                    match c.controller {
                        Controller::Player => {
                            // Do nothing, wait for input
                            game.turn_system.waiting_input = true;
                            game.camera.character_id = character_id;
                        }
                        Controller::Enemy => {
                            // Ai action
                            let position = &c.position;
                            let range = 5;
                            let mut humans: Vec<(&i32, &Character)> = game
                                .characters
                                .iter()
                                .filter(|(_, c)| {
                                    range >= Position::distance(&c.position, &position)
                                })
                                .filter(|(_, c)| c.race == Race::Human)
                                .filter(|(_, c)| !c.dead)
                                .collect();
                            humans.sort_by_key(|(_, c)| Position::distance(&c.position, &position));
                            if humans.is_empty() {
                                // idle / back
                                game.add_command(character_id, String::from("Idle"), Target::None);
                            } else {
                                // Yes => Move / Attack
                                // Player die => Attack / Move
                                let (&k, human) = humans.first().unwrap();
                                let dis = Position::distance(&c.position, &human.position);
                                if dis <= 1 {
                                    // attack
                                    if human.defeated {
                                        game.add_command(
                                            character_id,
                                            String::from("Idle"),
                                            Target::Character(k),
                                        );
                                    } else {
                                        game.add_command(
                                            character_id,
                                            String::from("Melee"),
                                            Target::Character(k),
                                        );
                                    }
                                } else {
                                    // move
                                    let x = human.position.x - c.position.x;
                                    let y = human.position.y - c.position.y;
                                    if x.abs() > y.abs() {
                                        let command = Character::move_command(
                                            game,
                                            character_id,
                                            if x.is_positive() { 1 } else { -1 },
                                            0,
                                        );
                                        game.set_character_command(character_id, command.unwrap());
                                    } else {
                                        let command = Character::move_command(
                                            game,
                                            character_id,
                                            0,
                                            if y.is_positive() { 1 } else { -1 },
                                        );
                                        game.set_character_command(character_id, command.unwrap());
                                    }
                                }
                            }
                        }
                        Controller::NPC => {
                            game.add_command(character_id, String::from("Idle"), Target::None);
                        }
                        Controller::Boss { waiting } => {
                            if waiting {
                                game.add_command(character_id, String::from("Idle"), Target::None);
                            } else {
                                if let Some(player_character_id) =
                                    game.floor_system.current.as_ref().map(|c| c.character_id)
                                {
                                    let target = game.characters.get(&player_character_id).unwrap();
                                    let dis = Position::distance(&c.position, &target.position);
                                    if dis <= 1 {
                                        // attack
                                        game.add_command(
                                            character_id,
                                            String::from("Melee"),
                                            Target::Character(player_character_id),
                                        );
                                    } else {
                                        // move
                                        let x = target.position.x - c.position.x;
                                        let y = target.position.y - c.position.y;
                                        if x.abs() > y.abs() {
                                            let command = Character::move_command(
                                                game,
                                                character_id,
                                                if x.is_positive() { 1 } else { -1 },
                                                0,
                                            );
                                            game.set_character_command(
                                                character_id,
                                                command.unwrap(),
                                            );
                                        } else {
                                            let command = Character::move_command(
                                                game,
                                                character_id,
                                                0,
                                                if y.is_positive() { 1 } else { -1 },
                                            );
                                            game.set_character_command(
                                                character_id,
                                                command.unwrap(),
                                            );
                                        }
                                    }
                                } else {
                                    log::error!("No player character id!");
                                    game.add_command(
                                        character_id,
                                        String::from("Idle"),
                                        Target::None,
                                    );
                                }
                            }
                        }
                    }
                    return (false, State::StartCommand);
                }
                TurnKind::Respawn { character_id } => {
                    // Respawn
                    let character = game.characters.get_mut(&character_id).unwrap();
                    character.hp.damage = 0;
                    character.mp.damage = 0;
                    character.attack.damage = 0;
                    character.defence.damage = 0;
                    character.sanity.damage = 0;
                    character.defeated = false;
                    character.dead = false;
                    character.hidden = false;
                    game.add_turn(Turn {
                        time: 1,
                        kind: TurnKind::Character { character_id },
                    });

                    game.turn_system.current_turn = None;

                    return (false, State::EndTurn);
                }
            }
        } else {
            log::warn!("start turn without turn");
            return (true, State::Empty);
        }
    }
    pub fn start_command(game: &mut Game) -> (bool, State) {
        let character_id = game
            .turn_system
            .current_turn
            .as_ref()
            .map(|t| match t.kind {
                TurnKind::Character { character_id } => character_id,
                TurnKind::Respawn { character_id: _ } => panic!(),
            })
            .unwrap();
        let character = game.characters.get_mut(&character_id).unwrap();
        if let Some(command_id) = character.command_id {
            Command::start(game, command_id);
            return (false, State::UpdateCommand);
        } else {
            // Wait for player input
            return (true, State::StartCommand);
        }
    }
    pub fn update_command(game: &mut Game, delta: f32) -> (bool, State) {
        let character_id = game
            .turn_system
            .current_turn
            .as_ref()
            .map(|t| match t.kind {
                TurnKind::Character { character_id } => character_id,
                TurnKind::Respawn { character_id: _ } => panic!(),
            })
            .unwrap();
        let character_map = game.characters.get(&character_id).unwrap().position.map_id;
        let camera_map = game
            .characters
            .get(&game.camera.character_id)
            .unwrap()
            .position
            .map_id;
        let character = game.characters.get_mut(&character_id).unwrap();
        if let Some(command_id) = character.command_id {
            let command = game.commands.get_mut(&command_id).unwrap();
            let wait_animation = character_map == camera_map;
            let duration = game.actions.get(&command.action_id).unwrap().duration;
            command.elapsed_time = command.elapsed_time + delta;
            let animation_end = command.elapsed_time >= duration;

            if wait_animation && !animation_end {
                Command::update(game, command_id);
                return (true, State::UpdateCommand);
            } else {
                return (false, State::EndCommand);
            }
        } else {
            log::warn!("Missing command");
            return (false, State::EndCommand);
        }
    }
    pub fn end_command(game: &mut Game) -> (bool, State) {
        let character_id = game
            .turn_system
            .current_turn
            .as_ref()
            .map(|t| match t.kind {
                TurnKind::Character { character_id } => character_id,
                TurnKind::Respawn { character_id: _ } => panic!(""),
            })
            .unwrap();
        let character = game.characters.get_mut(&character_id).unwrap();
        if let Some(command_id) = character.command_id {
            Command::end(game, command_id);
            game.commands.remove(&command_id);
        }
        let character = game.characters.get_mut(&character_id).unwrap();
        character.command_id = None;
        (false, State::EndTurn)
    }
    pub fn end_turn(game: &mut Game) -> (bool, State) {
        if let Some(mut t) = game.turn_system.current_turn.take() {
            match t.kind {
                TurnKind::Character { character_id } => {
                    t.time = 1;
                    game.add_turn(t);

                    let c = game.characters.get_mut(&character_id).unwrap();
                    c.crowd_controls.turn_end();
                }
                TurnKind::Respawn { character_id: _ } => {}
            }
        } else {
            log::warn!("Missing current turn");
        }
        (false, State::Empty)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TurnKind {
    Character { character_id: i32 },
    Respawn { character_id: i32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    Empty,
    StartTurn,
    StartCommand,
    UpdateCommand,
    EndCommand,
    EndTurn,
}
