use super::target_scene::TargetScene;
use crate::scene::{Data, Scene, Transition};
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use ggez::{event::KeyCode, nalgebra};

const BUTTON_W: i32 = 240;
const BUTTON_H: i32 = 180;
const BUTTON_ROW: usize = 4;

pub struct ActionScene {
    character_id: i32,
    current_item: usize,
    action_ids: Vec<i32>,
    waiting: bool,

    action_texts: Vec<graphics::Text>,
    button_rectangle: graphics::Mesh,
}

impl ActionScene {
    pub fn new(ctx: &mut Context, data: &Data, character_id: i32) -> Self {
        // Load/create resources such as images here.
        let mut action_texts = Vec::new();
        let mut action_ids = Vec::new();
        if let Some(character) = data.game.characters.get(&character_id) {
            for ca in character.character_action_ids.iter() {
                if let Some(action_id) = data.game.character_actions.get(ca).map(|ca| ca.action_id)
                {
                    if let Some(a) = data.game.actions.get(&action_id) {
                        // draw buttom
                        let s = format!(
                        " Name: {} \n Description: {} \n Targe: {:?} \n Cost: Hp({}) / MP({}) \n Range: {} \n Cooldown: {}",
                        a.name,
                        a.description,
                        a.target_kind,
                        a.hp.unwrap_or_default(),
                        a.mp.unwrap_or_default(),
                        a.range,
                        a.cooldown,
                    );
                        let text = graphics::Text::new(s);
                        action_texts.push(text);
                        action_ids.push(action_id);
                    }
                }
            }
        }
        let button_rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(0, 0, BUTTON_W, BUTTON_H),
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
            waiting: false,
        }
    }
}

impl Scene for ActionScene {
    fn update(&mut self, _ctx: &mut Context, data: &mut Data) -> GameResult<Transition> {
        if self.waiting {
            self.waiting = false;
            if data.target_scene.finish {
                return Ok(Transition::Pop);
            }
        }
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _data: &mut Data) -> GameResult<()> {
        if !self.waiting {
            // draw action menu
            for (i, text) in self.action_texts.iter().enumerate() {
                let x = (i / BUTTON_ROW * (BUTTON_W as usize)) as f32;
                let y = (i % BUTTON_ROW * (BUTTON_H as usize)) as f32;
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
                graphics::Rect::new_i32(0, 0, BUTTON_W, BUTTON_H),
                // graphics::BLACK,
                graphics::Color::from_rgb(255, 255, 0),
            )?;
            graphics::draw(
                ctx,
                &highlight,
                graphics::DrawParam::new().dest(nalgebra::Point2::new(
                    (self.current_item / BUTTON_ROW * (BUTTON_W as usize)) as f32,
                    (self.current_item % BUTTON_ROW * (BUTTON_H as usize)) as f32,
                )),
            )?;
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
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
                if self.current_item >= BUTTON_ROW {
                    self.current_item -= BUTTON_ROW;
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
                if self.current_item + BUTTON_ROW < self.action_texts.len() {
                    self.current_item += BUTTON_ROW;
                }
            }
            KeyCode::Q => {
                // Cancel
                data.action_scene.action_id = None;
                return Transition::Pop;
            }
            KeyCode::E => {
                // Confirm
                if let Some(action_ids) = self.action_ids.get(self.current_item) {
                    self.waiting = true;
                    data.target_scene.finish = false;
                    return Transition::Push(Box::new(TargetScene::new(
                        _ctx,
                        data,
                        self.character_id,
                        *action_ids,
                    )));
                }
            }
            KeyCode::Escape => {
                // Open menu
            }
            _ => {}
        }

        Transition::None
    }
}
