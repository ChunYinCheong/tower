use ggez::{graphics, Context, GameResult};
use tower::core::Character;

pub struct Stats {
    pub text: graphics::Text,
    pub canvas: graphics::Canvas,
}

impl Stats {
    pub fn new(ctx: &mut Context, character: &Character) -> GameResult<Self> {
        let text = graphics::Text::new(format!(
            "Hp: {}/{} \nMp: {}/{} \nAttack: {} \nDefence: {}",
            character.hp.current(),
            character.hp.max(),
            character.mp.current(),
            character.mp.max(),
            character.attack.current(),
            character.defence.current(),
        ));
        let canvas = graphics::Canvas::with_window_size(ctx)?;
        Ok(Self { text, canvas })
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam {
                dest: ggez::mint::Point2 { x: 0.0, y: 0.0 },
                ..Default::default()
            },
        )?;

        Ok(())
    }
    pub fn draw_canvas(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, graphics::Color::from((0, 0, 0, 0)));

        self.draw(ctx)?;

        graphics::set_canvas(ctx, None);

        Ok(())
    }
}
