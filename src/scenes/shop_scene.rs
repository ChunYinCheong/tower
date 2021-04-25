use super::pause_scene::PauseScene;
use crate::scene::{Data, Scene, Transition};
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use ggez::{event::KeyMods, timer};

pub struct ShopScene {
    current_item: usize,
}

impl ShopScene {
    pub fn new(ctx: &mut Context, data: &mut Data) -> Self {
        // Load/create resources such as images here.
        Self { current_item: 0 }
    }
}

impl Scene for ShopScene {
    fn update(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<Transition> {
        data.game.update(timer::delta(&ctx).as_secs_f32());

        if data.game.shop_system.current.is_none() {
            return Ok(Transition::Pop);
        }

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<()> {
        if let Some(current) = &data.game.shop_system.current {
            let bg = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(0, 0, 256, 64),
                // graphics::BLACK,
                graphics::Color::from_rgba(0, 0, 127, 127),
            )?;
            if let Some(items) = data
                .game
                .characters
                .get(&current.shop_character_id)
                .and_then(|c| c.shop.as_ref())
                .map(|shop| &shop.items)
            {
                for (i, s) in items.iter().enumerate() {
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
                    let name = data.game.items.get(&s.item_id).map(|i| &i.name).unwrap();
                    let text = graphics::Text::new(String::from(name));
                    graphics::draw(
                        ctx,
                        &text,
                        graphics::DrawParam {
                            dest: ggez::mint::Point2 { x, y },
                            ..Default::default()
                        },
                    )?;
                }
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

            // Description
            if let Some(item) = data
                .game
                .characters
                .get(&current.shop_character_id)
                .and_then(|c| c.shop.as_ref())
                .map(|shop| &shop.items)
                .map(|items| items.get(self.current_item))
                .flatten()
            {
                let quantity = data
                    .game
                    .characters
                    .get(&current.character_id)
                    .map(|c| c.items.iter().filter(|i| i.item_id == item.item_id).count())
                    .unwrap_or_default();
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
                let description = data
                    .game
                    .items
                    .get(&item.item_id)
                    .map(|i| &i.description)
                    .unwrap();

                let text = graphics::Text::new(format!(
                    "You have: {} \nDescription: {}",
                    quantity, description
                ));
                graphics::draw(
                    ctx,
                    &text,
                    graphics::DrawParam {
                        dest: ggez::mint::Point2 { x, y },
                        ..Default::default()
                    },
                )?;
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
                    .shop_system
                    .current
                    .as_ref()
                    .and_then(|current| data.game.characters.get(&current.shop_character_id))
                    .and_then(|c| c.shop.as_ref())
                    .map(|shop| shop.items.len())
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
                data.game.close_shop();
                if data.game.shop_system.current.is_none() {
                    return Transition::Pop;
                }
            }
            KeyCode::E => {
                // Confirm
                data.game.buy_item(self.current_item);
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
