use serde::{Deserialize, Serialize};

use super::{sprite::Animation, Game, Position};

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub hp: Option<i32>,
    pub mp: Option<i32>,
    pub action_kind: ActionKind,
    pub target_kind: TargetKind,
    pub effect_kind: EffectKind,
    pub range: i32,
    pub duration: f32,
    pub cooldown: i32,
    pub animation: Option<Animation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionKind {
    Idle,
    Talk,
    Move,
    Damage(i32),
    Drain,
    Lullaby { duration: i32 },
    Thunder { duration: i32 },
    Root(i32),
    HpRecover(i32),
    MpRecover(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
    None,
    Character(i32),
    Position(Position),
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum TargetKind {
    None,
    Character,
    Position,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum EffectKind {
    Character,
    Square(i32),
    Manhattan(i32),
    // Circle(f32),
}

// None
// SelfRange
// Character
// CharacterRange
// Position
// PositionRange
// Direction

impl EffectKind {
    pub fn effective_characters(
        game: &Game,
        character_id: i32,
        target: &Target,
        effect_kind: EffectKind,
    ) -> Vec<i32> {
        match effect_kind {
            EffectKind::Character => match target {
                Target::None => vec![character_id],
                Target::Character(id) => vec![*id],
                Target::Position(_) => panic!(),
            },
            EffectKind::Square(range) => {
                let position = match target {
                    Target::None => game
                        .characters
                        .get(&character_id)
                        .map(|c| c.position)
                        .unwrap(),
                    Target::Character(id) => game.characters.get(id).map(|c| c.position).unwrap(),
                    Target::Position(p) => *p,
                };
                game.characters_in_square(&position, range)
            }
            EffectKind::Manhattan(range) => {
                let position = match target {
                    Target::None => game
                        .characters
                        .get(&character_id)
                        .map(|c| c.position)
                        .unwrap(),
                    Target::Character(id) => game.characters.get(id).map(|c| c.position).unwrap(),
                    Target::Position(p) => *p,
                };
                game.characters_in_range(&position, range)
            }
        }
    }
}
