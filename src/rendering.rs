use ggez::{
    graphics::{self, Drawable, Image},
    Context, GameResult,
};
use tower::core::{Character, Game, Position};

pub struct Rendering;

#[derive(Default)]
pub struct RenderingParam {
    dest: Option<Destination>,
    rect: Option<RectSetting>,
    target: Option<TargetSize>,
}
struct Destination {
    pub map_id: i32,
    pub x: f32,
    pub y: f32,
}

struct RectSetting {
    pub rect_x: f32,
    pub rect_y: f32,
    pub rect_width: f32,
    pub rect_height: f32,
}

struct TargetSize {
    pub target_width: f32,
    pub target_height: f32,
}

impl RenderingParam {
    pub fn dest(mut self, map_id: i32, x: f32, y: f32) -> Self {
        self.dest = Some(Destination { map_id, x, y });
        self
    }
    pub fn position(mut self, position: &Position) -> Self {
        self.dest = Some(Destination {
            map_id: position.map_id,
            x: position.x as f32,
            y: position.y as f32,
        });
        self
    }

    pub fn rect(mut self, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> Self {
        self.rect = Some(RectSetting {
            rect_x,
            rect_y,
            rect_width,
            rect_height,
        });
        self
    }
    pub fn target_size(mut self, x: f32, y: f32) -> Self {
        self.target = Some(TargetSize {
            target_width: x,
            target_height: y,
        });
        self
    }

    pub fn draw_image(&self, game: &Game, image: &Image, ctx: &mut Context) -> GameResult {
        if let Some(character) = game.characters.get(&game.camera.character_id) {
            let mut param = graphics::DrawParam::default();
            if let Some(dest) = &self.dest {
                if dest.map_id != character.position.map_id {
                    return Ok(());
                }
                if (character.position.x as f32 - dest.x).abs() > game.camera.extend as f32
                    || (character.position.y as f32 - dest.y).abs() > game.camera.extend as f32
                {
                    return Ok(());
                }

                if let Some(dest) = Rendering::dest(game, dest.x as f32, dest.y as f32) {
                    param = param.dest(dest);
                }
            }

            if let Some(rect) = &self.rect {
                let image_width = image.width() as f32;
                let image_height = image.height() as f32;
                let fx = rect.rect_x / image_width;
                let fy = rect.rect_y / image_height;
                let w = rect.rect_width as f32 / image_width;
                let h = rect.rect_height as f32 / image_height;
                let rect = graphics::Rect::new(fx, fy, w, h);
                param = param.src(rect);
            }

            if let Some(target) = &self.target {
                let scale = match &self.rect {
                    Some(rect) => [
                        target.target_width / rect.rect_width,
                        target.target_height / rect.rect_height,
                    ],
                    None => [
                        target.target_width / image.width() as f32,
                        target.target_height / image.height() as f32,
                    ],
                };
                param = param.scale(scale);
            }

            graphics::draw(ctx, image, param)?;
        }
        Ok(())
    }
}

impl Rendering {
    fn dest(game: &Game, x: f32, y: f32) -> Option<ggez::mint::Point2<f32>> {
        if let Some(character) = game.characters.get(&game.camera.character_id) {
            Some(ggez::mint::Point2 {
                x: (((-character.position.x + game.camera.extend) as f32 + x)
                    * game.camera.tile_size as f32
                    + game.camera.border as f32) as f32,
                y: (((-character.position.y + game.camera.extend) as f32 + y)
                    * game.camera.tile_size as f32
                    + game.camera.border as f32) as f32,
            })
        } else {
            None
        }
    }

    fn draw<D>(
        ctx: &mut Context,
        drawable: &D,
        map_id: i32,
        x: f32,
        y: f32,
        game: &Game,
    ) -> GameResult<()>
    where
        D: Drawable,
    {
        if let Some(character) = game.characters.get(&game.camera.character_id) {
            if map_id != character.position.map_id {
                return Ok(());
            }
            if (character.position.x as f32 - x).abs() > game.camera.extend as f32
                || (character.position.y as f32 - y).abs() > game.camera.extend as f32
            {
                return Ok(());
            }
            if let Some(dest) = Rendering::dest(game, x as f32, y as f32) {
                graphics::draw(
                    ctx,
                    drawable,
                    graphics::DrawParam {
                        dest,
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }

    pub fn draw_at_position<D>(
        ctx: &mut Context,
        drawable: &D,
        position: &Position,
        game: &Game,
    ) -> GameResult<()>
    where
        D: Drawable,
    {
        Rendering::draw(
            ctx,
            drawable,
            position.map_id,
            position.x as f32,
            position.y as f32,
            game,
        )?;
        Ok(())
    }

    pub fn draw_character_border(
        ctx: &mut Context,
        character: &Character,
        game: &Game,
    ) -> GameResult<()> {
        let highlight = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            graphics::Rect::new_i32(0, 0, 64, 64),
            graphics::Color::from_rgb(0, 255, 0),
        )?;

        Rendering::draw(
            ctx,
            &highlight,
            character.position.map_id,
            character.position.x as f32 + character.offset_x,
            character.position.y as f32 + character.offset_y,
            game,
        )?;
        Ok(())
    }

    pub fn draw_border(ctx: &mut Context, game: &Game) -> GameResult<()> {
        // Draw Border
        let size = (game.camera.extend * 2 + 1) * game.camera.tile_size;
        let map_border = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(game.camera.border as f32),
            graphics::Rect::new_i32(
                game.camera.border / 2,
                game.camera.border / 2,
                size + game.camera.border,
                size + game.camera.border,
            ),
            // graphics::BLACK,
            graphics::Color::from_rgb(255, 0, 0),
        )?;
        graphics::draw(ctx, &map_border, graphics::DrawParam::new())?;
        Ok(())
    }
}
