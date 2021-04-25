use ggez::Context;
use ggez::GameResult;

use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::graphics;

use crate::scene::{Data, Scene, Transition};

pub struct PauseScene {}

impl PauseScene {
    pub fn new(_ctx: &mut Context, _data: &Data) -> Self {
        Self {}
    }
}

impl Scene for PauseScene {
    fn update(&mut self, _ctx: &mut Context, _data: &mut Data) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _data: &mut Data) -> GameResult<()> {
        let bg = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(0, 0, 1280, 720),
            graphics::Color::from_rgba(0, 0, 0, 127),
        )?;
        graphics::draw(ctx, &bg, graphics::DrawParam::new())?;

        let text = graphics::Text::new("Press N to create new character");
        graphics::draw(ctx, &text, graphics::DrawParam::default())?;
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
            }
            KeyCode::A => {
                // Left
            }
            KeyCode::S => {
                // Down
            }
            KeyCode::D => {
                // Right
            }
            KeyCode::Q => {
                // Cancel
                return Transition::Pop;
            }
            KeyCode::E => {
                // Confirm
            }
            KeyCode::N => {
                data.game.new_player_character();
                return Transition::Pop;
            }
            KeyCode::Escape => {
                return Transition::Pop;
            }
            _ => {}
        }

        Transition::None
    }
}
