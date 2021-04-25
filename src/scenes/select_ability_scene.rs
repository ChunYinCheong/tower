use super::pause_scene::PauseScene;
use crate::scene::{Data, Scene, Transition};
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use ggez::{event::KeyMods, nalgebra, timer};

pub struct SelectAbilityScene {
    character_id: i32,
    current_item: usize,
    action_ids: Vec<i32>,
    action_texts: Vec<graphics::Text>,
    button_rectangle: graphics::Mesh,
}

impl SelectAbilityScene {
    pub fn new(ctx: &mut Context, data: &mut Data, character_id: i32) -> Self {
        let mut action_texts = Vec::new();
        let mut target_kinds = Vec::new();
        let mut action_ids = Vec::new();
        if let Some(character) = data.game.characters.get(&character_id) {
            for ca in character.character_action_ids.iter() {
                if let Some(action_id) = data.game.character_actions.get(ca).map(|ca| ca.action_id)
                {
                    if let Some(a) = data.game.actions.get(&action_id) {
                        // draw buttom
                        let text = graphics::Text::new(&a.name[..]);
                        action_texts.push(text);
                        target_kinds.push(a.target_kind);
                        action_ids.push(action_id);
                    }
                }
            }
        }
        let button_rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(0, 0, 256, 128),
            // graphics::BLACK,
            graphics::Color::from_rgb(0, 127, 127),
        )
        .unwrap();
        Self {
            character_id,
            current_item: 0,
            action_texts,
            action_ids,
            button_rectangle,
        }
    }
}

impl Scene for SelectAbilityScene {
    fn update(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<Transition> {
        if !data
            .game
            .novel_system
            .current
            .as_ref()
            .map(|c| c.waiting_select_ability)
            .unwrap_or(false)
        {
            return Ok(Transition::Pop);
        }

        data.game.update(timer::delta(&ctx).as_secs_f32());
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<()> {
        // draw action menu
        for (i, text) in self.action_texts.iter().enumerate() {
            let x = (i / 5 * 256) as f32;
            let y = (i % 5 * 128) as f32;
            graphics::draw(
                ctx,
                &self.button_rectangle,
                graphics::DrawParam::new().dest(nalgebra::Point2::new(x, y)),
            )?;
            graphics::draw(
                ctx,
                text,
                graphics::DrawParam::new().dest(nalgebra::Point2::new(x, y)),
            )?;
        }
        // Draw selection
        let highlight = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(4.0),
            graphics::Rect::new_i32(0, 0, 256, 128),
            // graphics::BLACK,
            graphics::Color::from_rgb(255, 255, 0),
        )?;
        graphics::draw(
            ctx,
            &highlight,
            graphics::DrawParam::new().dest(nalgebra::Point2::new(
                (self.current_item / 5 * 256) as f32,
                (self.current_item % 5 * 128) as f32,
            )),
        )?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
        data: &mut Data,
    ) -> Transition {
        match keycode {
            KeyCode::W => {
                // Up
                if self.current_item >= 1 {
                    self.current_item -= 1;
                }
            }
            KeyCode::A => {
                // Left
                if self.current_item >= 5 {
                    self.current_item -= 5;
                }
            }
            KeyCode::S => {
                // Down
                if self.current_item + 1 < self.action_texts.len() {
                    self.current_item += 1;
                }
            }
            KeyCode::D => {
                // Right
                if self.current_item + 5 < self.action_texts.len() {
                    self.current_item += 5;
                }
            }
            KeyCode::Q => {
                // Cancel
                data.game.cancel_select_ability();
                // return Transition::Pop;
            }
            KeyCode::E => {
                // Confirm
                data.game.select_ability(self.current_item);
            }
            KeyCode::Escape => {
                // Player can open pause menu anytime
                return Transition::Push(Box::new(PauseScene::new(ctx, &data)));
            }
            _ => {}
        }

        Transition::None
    }
}
