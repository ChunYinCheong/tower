use crate::{
    rendering::Rendering,
    scene::{Data, Scene, Transition},
    ui::stats::Stats,
};
use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use tower::core::{Command, Position, Target, TargetKind};

pub struct TargetScene {
    character_id: i32,
    action_id: i32,

    position: Position,
}

impl TargetScene {
    pub fn new(_ctx: &mut Context, data: &Data, character_id: i32, action_id: i32) -> Self {
        // Load/create resources such as images here.
        let position = data
            .game
            .characters
            .get(&character_id)
            .map(|c| c.position)
            .unwrap();
        Self {
            character_id,
            action_id,
            position,
        }
    }
}

impl Scene for TargetScene {
    fn update(&mut self, _ctx: &mut Context, _data: &mut Data) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<()> {
        let tile_rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(0, 0, 64, 64),
            graphics::Color::from_rgba(0, 0, 0, 127),
        )?;
        Rendering::draw_at_position(ctx, &tile_rectangle, &self.position, &data.game)?;

        if let Some(id) = data.game.character_at_position(&self.position) {
            let character = data.game.characters.get(&id).unwrap();

            let stats_ui = Stats::new(ctx, character)?;
            stats_ui.draw_canvas(ctx)?;
            graphics::draw(
                ctx,
                &stats_ui.canvas,
                graphics::DrawParam {
                    dest: ggez::mint::Point2 { x: 720.0, y: 360.0 },
                    ..Default::default()
                },
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
                self.position.y -= 1;
            }
            KeyCode::A => {
                // Left
                self.position.x -= 1;
            }
            KeyCode::S => {
                // Down
                self.position.y += 1;
            }
            KeyCode::D => {
                // Right
                self.position.x += 1;
            }
            KeyCode::Q => {
                // Cancel
                data.target_scene.finish = false;
                return Transition::Pop;
            }
            KeyCode::E => {
                // Confirm
                let target_kind = data
                    .game
                    .actions
                    .get(&self.action_id)
                    .map(|a| a.target_kind)
                    .unwrap();

                let target = match target_kind {
                    TargetKind::None => Some(Target::None),
                    TargetKind::Character => {
                        match data.game.character_at_position(&self.position) {
                            Some(id) => Some(Target::Character(id)),
                            None => None,
                        }
                    }
                    TargetKind::Position => Some(Target::Position(self.position)),
                };
                if let Some(target) = target {
                    if let Ok(command) =
                        Command::new(&mut data.game, self.character_id, self.action_id, target)
                    {
                        data.game.add_player_command(self.character_id, command);
                        data.target_scene.finish = true;
                        return Transition::Pop;
                    }
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
