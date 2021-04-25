use super::{ActionKind, AnimationEffect, Character, EffectKind, Game, Position, Target};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub character_id: i32,
    pub action_id: i32,
    pub target: Target,
    pub elapsed_time: f32,
    pub data: CommandData,
    pub sprite_animation_id: Option<i32>,
    pub state: CommandState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandState {
    New,
    Started,
    Ended,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandData {
    None,
    Move { from: Position, to: Position },
    Drain { sprite_animation_id: i32 },
}

impl Command {
    pub fn new(
        game: &mut Game,
        character_id: i32,
        action_id: i32,
        target: Target,
    ) -> Result<Self, CommandError> {
        let action = game.actions.get(&action_id).unwrap();
        let character = game.characters.get(&character_id).unwrap();
        let data = match &action.action_kind {
            ActionKind::Idle => CommandData::None,
            ActionKind::Talk => CommandData::None,
            ActionKind::Move => {
                if let Target::Position(position) = target {
                    CommandData::Move {
                        from: character.position,
                        to: position,
                    }
                } else {
                    return Err(CommandError::Target);
                }
            }
            ActionKind::Damage(_) => CommandData::None,
            ActionKind::Drain => {
                if let Target::Character(id) = target {
                    let target = game.characters.get(&id).unwrap();
                    let animation = game
                        .character_sprites
                        .get(&character.character_sprite_id)
                        .and_then(|cs| cs.animation.get("drain"))
                        .unwrap()
                        .clone();

                    let sprite_animation_id = 1;
                    game.sprite_animations.insert(
                        sprite_animation_id,
                        AnimationEffect {
                            id: sprite_animation_id,
                            animation,
                            percentage: 0.0,
                            position: target.position,
                        },
                    );
                    CommandData::Drain {
                        sprite_animation_id,
                    }
                } else {
                    return Err(CommandError::Target);
                }
            }
            ActionKind::Lullaby { duration: _ } => CommandData::None,
            ActionKind::Thunder { duration: _ } => CommandData::None,
            ActionKind::Root(_) => CommandData::None,
            ActionKind::HpRecover(_) => CommandData::None,
            ActionKind::MpRecover(_) => CommandData::None,
        };

        let sprite_animation_id = match &action.animation {
            Some(animation) => {
                let id = 1;
                game.sprite_animations.insert(
                    id,
                    AnimationEffect {
                        id,
                        animation: animation.clone(),
                        percentage: 0.0,
                        position: match target {
                            Target::None => game
                                .characters
                                .get(&character_id)
                                .map(|c| c.position)
                                .unwrap(),
                            Target::Character(target_id) => {
                                game.characters.get(&target_id).map(|c| c.position).unwrap()
                            }
                            Target::Position(position) => position,
                        },
                    },
                );
                Some(id)
            }
            None => None,
        };
        Ok(Self {
            character_id,
            action_id,
            target,
            elapsed_time: 0.0,
            data,
            sprite_animation_id,
            state: CommandState::New,
        })
    }
    pub fn start(game: &mut Game, command_id: i32) {
        let command = game.commands.get_mut(&command_id).unwrap();
        command.state = CommandState::Started;
        let action_id = command.action_id;
        let (hp, mp) = game.actions.get(&action_id).map(|a| (a.hp, a.mp)).unwrap();
        let character = game.characters.get_mut(&command.character_id).unwrap();
        if hp.is_some() {
            character.hp.damage += hp.unwrap();
        }
        if mp.is_some() {
            character.mp.damage += mp.unwrap();
        }
    }
    pub fn end(game: &mut Game, command_id: i32) {
        let command = game.commands.get_mut(&command_id).unwrap();
        command.state = CommandState::Ended;

        let command = game.commands.get(&command_id).unwrap();
        if let Some(sprite_animation_id) = command.sprite_animation_id {
            game.sprite_animations.remove(&sprite_animation_id);
        }

        let action_id = command.action_id;
        if let Some(action) = game.actions.get(&action_id) {
            match &action.action_kind {
                ActionKind::Idle => {}
                ActionKind::Talk => {
                    // Dialog time...
                    let target_id = match command.target {
                        Target::None => panic!(),
                        Target::Character(target_id) => target_id,
                        Target::Position(_) => panic!(),
                    };
                    let character_id = command.character_id;
                    let id = game.characters.get(&target_id).map(|c| c.talk_id).unwrap();
                    game.start_novel(id, character_id, target_id);
                }
                ActionKind::Move => match command.data {
                    CommandData::Move { from, to } => {
                        let character_id = command.character_id;
                        let is_walkable = game
                            .tile_maps
                            .get(&to.map_id)
                            .map(|m| &m.tiles)
                            .and_then(|tiles| tiles.get(to.y as usize))
                            .and_then(|r| r.get(to.x as usize))
                            .and_then(|id| game.tiles.get(id))
                            .map(|t| t.walkable)
                            .unwrap_or(false);
                        let is_collide = game.character_at_position(&to).is_some();
                        let pos = if !is_walkable || is_collide { from } else { to };
                        let c = game.characters.get_mut(&character_id).unwrap();
                        c.offset_x = 0.0;
                        c.offset_y = 0.0;
                        game.character_set_position(character_id, pos);
                    }
                    _ => panic!(),
                },
                ActionKind::Damage(damage) => {
                    let damage = *damage;
                    let character_id = command.character_id;
                    let characters = EffectKind::effective_characters(
                        game,
                        character_id,
                        &command.target,
                        action.effect_kind,
                    );
                    for target_id in characters {
                        Character::take_damage(game, damage, target_id, character_id);
                    }
                }
                ActionKind::Drain => {
                    match command.data {
                        CommandData::Drain {
                            sprite_animation_id,
                        } => {
                            game.sprite_animations.remove(&sprite_animation_id);
                        }
                        _ => panic!(),
                    }

                    let character_id = command.character_id;
                    let characters = EffectKind::effective_characters(
                        game,
                        character_id,
                        &command.target,
                        action.effect_kind,
                    );
                    for target_id in characters {
                        let target = game.characters.get_mut(&target_id).unwrap();
                        if target.experience > 0 {
                            target.gain_exp(-1);
                            let character = game.characters.get_mut(&character_id).unwrap();
                            character.gain_exp(1);
                        }
                    }
                }
                ActionKind::Lullaby { duration } => {
                    let character_id = command.character_id;
                    let characters = EffectKind::effective_characters(
                        game,
                        character_id,
                        &command.target,
                        action.effect_kind,
                    );
                    for target_id in characters {
                        let target = game.characters.get_mut(&target_id).unwrap();
                        target.crowd_controls.sleep = target.crowd_controls.sleep.max(*duration);
                    }
                }
                ActionKind::Thunder { duration } => {
                    let character_id = command.character_id;
                    let characters = EffectKind::effective_characters(
                        game,
                        character_id,
                        &command.target,
                        action.effect_kind,
                    );
                    for target_id in characters {
                        let target = game.characters.get_mut(&target_id).unwrap();
                        target.crowd_controls.shock = target.crowd_controls.shock.max(*duration);
                        target.crowd_controls.shocked = false;
                    }
                }
                ActionKind::Root(duration) => {
                    let character_id = command.character_id;
                    let characters = EffectKind::effective_characters(
                        game,
                        character_id,
                        &command.target,
                        action.effect_kind,
                    );
                    for target_id in characters {
                        let target = game.characters.get_mut(&target_id).unwrap();
                        target.crowd_controls.root = target.crowd_controls.root.max(*duration);
                    }
                }
                ActionKind::HpRecover(i) => {
                    let character_id = command.character_id;
                    let characters = EffectKind::effective_characters(
                        game,
                        character_id,
                        &command.target,
                        action.effect_kind,
                    );
                    for target_id in characters {
                        let target = game.characters.get_mut(&target_id).unwrap();
                        target.hp.damage -= *i;
                    }
                }
                ActionKind::MpRecover(i) => {
                    let character_id = command.character_id;
                    let characters = EffectKind::effective_characters(
                        game,
                        character_id,
                        &command.target,
                        action.effect_kind,
                    );
                    for target_id in characters {
                        let target = game.characters.get_mut(&target_id).unwrap();
                        target.mp.damage -= *i;
                    }
                }
            }
        }
    }

    pub fn update(game: &mut Game, command_id: i32) {
        let command = game.commands.get(&command_id).unwrap();
        let duration = { game.actions.get(&command.action_id).unwrap().duration };
        let percentage = (command.elapsed_time / duration).min(1.0);

        if let Some(sprite_animation_id) = &command.sprite_animation_id {
            let animation = game
                .sprite_animations
                .get_mut(&sprite_animation_id)
                .unwrap();
            animation.percentage = percentage;
        }
        match command.data {
            CommandData::None => {}
            CommandData::Move { from, to } => {
                let character_id = command.character_id;
                let x = (to.x - from.x) as f32 * percentage;
                let y = (to.y - from.y) as f32 * percentage;
                let c = game.characters.get_mut(&character_id).unwrap();
                c.offset_x = x;
                c.offset_y = y;
            }
            CommandData::Drain {
                sprite_animation_id,
            } => {
                let animation = game
                    .sprite_animations
                    .get_mut(&sprite_animation_id)
                    .unwrap();
                animation.percentage = percentage;
            }
        }
    }
}
