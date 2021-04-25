use ggez::Context;
use ggez::{event::KeyCode, GameResult};
use ggez::{event::KeyMods, graphics};

pub use crate::data::Data;

pub trait Scene {
    fn update(&mut self, _ctx: &mut Context, data: &mut Data) -> GameResult<Transition>;
    fn draw(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult;

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
        data: &mut Data,
    ) -> Transition;
}

pub enum Transition {
    None,
    Pop,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
}

pub struct SceneStack {
    pub stack: Vec<Box<dyn Scene>>,
}

impl SceneStack {
    pub fn new() -> Self {
        SceneStack {
            stack: Default::default(),
        }
    }

    fn do_transition(&mut self, tran: Transition) {
        match tran {
            Transition::None => {}
            Transition::Pop => {
                self.stack.pop();
            }
            Transition::Push(s) => {
                self.stack.push(s);
            }
            Transition::Replace(s) => {
                self.stack.clear();
                self.stack.push(s);
            }
        }
    }

    pub fn update(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult {
        // let mut trans = vec![];
        // for scene in &mut self.stack {
        //     let t = scene.update(ctx, data)?;
        //     trans.push(t);
        // }
        // for tran in trans {
        //     self.do_transition(tran);
        // }

        if let Some(scene) = self.stack.last_mut() {
            let tran = scene.update(ctx, data)?;
            self.do_transition(tran);
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for scene in &mut self.stack {
            scene.draw(ctx, data)?;
        }

        graphics::present(ctx)?;

        Ok(())
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
        data: &mut Data,
    ) {
        if let Some(scene) = self.stack.last_mut() {
            let tran = scene.key_down_event(ctx, keycode, keymods, repeat, data);
            self.do_transition(tran);
        }
    }
}
