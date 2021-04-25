use super::{
    pause_scene::PauseScene, select_ability_scene::SelectAbilityScene, shop_scene::ShopScene,
};
use crate::scene::{Data, Scene, Transition};
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use ggez::{event::KeyMods, timer};

pub struct NovelScene {
    pub text: Option<graphics::Text>,
    current_item: usize,
}

impl NovelScene {
    pub fn new(ctx: &mut Context, data: &mut Data) -> Self {
        // Load/create resources such as images here.
        let text = None;
        Self {
            text,
            current_item: 0,
        }
    }
}

impl Scene for NovelScene {
    fn update(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<Transition> {
        if data.game.shop_system.current.is_some() {
            return Ok(Transition::Push(Box::new(ShopScene::new(ctx, data))));
        }
        if data.game.novel_system.current.is_none() {
            return Ok(Transition::Pop);
        }
        if let Some(current) = &data.game.novel_system.current {
            if current.waiting_select_ability {
                return Ok(Transition::Push(Box::new(SelectAbilityScene::new(
                    ctx,
                    data,
                    current.character_id,
                ))));
            }
        }
        data.game.update(timer::delta(&ctx).as_secs_f32());

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<()> {
        if data.game.novel_system.pause {
            return Ok(());
        }
        if let Some(current) = &data.game.novel_system.current {
            // Backgroud
            if let Some(background) = &current.background {
                match *background {
                    tower::core::Background::None => {}
                    tower::core::Background::Color(r, g, b, a) => {
                        let mesh = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect::new_i32(0, 0, 1280, 720),
                            graphics::Color::from_rgba(r, g, b, a),
                        )?;
                        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
                    }
                    tower::core::Background::Image(ref s) => {
                        let image = data.image_caches.get(ctx, s)?;
                        graphics::draw(ctx, image, graphics::DrawParam::default())?;
                    }
                }
            }
            // Text
            let bg = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(0, 0, 256, 64),
                // graphics::BLACK,
                graphics::Color::from_rgba(0, 0, 127, 127),
            )?;
            if let Some(s) = current.history.last() {
                let x = 0.0;
                let y = 0.0;
                graphics::draw(
                    ctx,
                    &bg,
                    graphics::DrawParam {
                        dest: ggez::mint::Point2 { x, y },
                        ..Default::default()
                    },
                )?;
                // draw text
                let text = graphics::Text::new(String::from(s));
                graphics::draw(
                    ctx,
                    &text,
                    graphics::DrawParam {
                        dest: ggez::mint::Point2 { x, y },
                        ..Default::default()
                    },
                )?;
            }

            // Select
            if let Some(select) = &current.select {
                for (i, s) in select.iter().enumerate() {
                    let x = 0.0;
                    let y = ((i + 1) * 64) as f32;
                    graphics::draw(
                        ctx,
                        &bg,
                        graphics::DrawParam {
                            dest: ggez::mint::Point2 { x, y },
                            ..Default::default()
                        },
                    )?;
                    let text = graphics::Text::new(String::from(s));
                    graphics::draw(
                        ctx,
                        &text,
                        graphics::DrawParam {
                            dest: ggez::mint::Point2 { x, y },
                            ..Default::default()
                        },
                    )?;
                }

                // Draw selection
                let highlight = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(4.0),
                    graphics::Rect::new_i32(0, 0, 256, 64),
                    // graphics::BLACK,
                    graphics::Color::from_rgb(255, 255, 0),
                )?;
                graphics::draw(
                    ctx,
                    &highlight,
                    graphics::DrawParam {
                        dest: ggez::mint::Point2 {
                            x: 0.0,
                            y: ((self.current_item + 1) * 64) as f32,
                        },
                        ..Default::default()
                    },
                )?;
            } else {
                self.current_item = 0;
            }
        }
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
                if self.current_item > 0 {
                    self.current_item -= 1;
                }
            }
            KeyCode::A => {
                // Left
            }
            KeyCode::S => {
                // Down
                let m = data
                    .game
                    .novel_system
                    .current
                    .as_ref()
                    .and_then(|c| c.select.as_ref())
                    .map(|s| s.len())
                    .unwrap_or_default();
                if self.current_item + 1 < m {
                    self.current_item += 1;
                } else {
                    self.current_item = 0;
                }
            }
            KeyCode::D => {
                // Right
            }
            KeyCode::Q => {
                // Cancel
            }
            KeyCode::E => {
                // Confirm
                if let Some(select) = data
                    .game
                    .novel_system
                    .current
                    .as_ref()
                    .and_then(|c| c.select.as_ref())
                {
                    // In selection
                    // Check index
                    if self.current_item < select.len() {
                        // do select
                        data.game.select_option(self.current_item);
                    }
                } else {
                    // Next
                    data.game.novel_run();
                }

                if data.game.novel_system.current.is_none() {
                    return Transition::Pop;
                }
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
