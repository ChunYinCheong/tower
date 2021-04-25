use crate::{
    rendering::{Rendering, RenderingParam},
    ui::stats::Stats,
    ActionScene,
};
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use ggez::{event::KeyMods, timer};
use tower::core::{Animation, Character, TileKind, TurnKind};

use crate::scene::{Data, Scene, Transition};

use super::{novel_scene::NovelScene, pause_scene::PauseScene};

pub struct LevelScene {}

impl LevelScene {
    pub fn new(_ctx: &mut Context, _data: &mut Data) -> Self {
        // Load/create resources such as images here.
        Self {}
    }
}

impl Scene for LevelScene {
    fn update(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<Transition> {
        if data.game.novel_system.current.is_some() {
            return Ok(Transition::Push(Box::new(NovelScene::new(ctx, data))));
        }

        data.game.update(timer::delta(&ctx).as_secs_f32());

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut Data) -> GameResult<()> {
        let camera = &data.game.camera;
        if let Some(character) = data.game.characters.get(&camera.character_id) {
            // Draw Tiles
            if let Some(map) = data.game.tile_maps.get(&character.position.map_id) {
                let start_y = character.position.y - camera.extend;
                let end_y = character.position.y + camera.extend;
                let start_x = character.position.x - camera.extend;
                let end_x = character.position.x + camera.extend;

                for y in start_y..=end_y {
                    if let Some(rows) = map.tiles.get(y as usize) {
                        for x in start_x..=end_x {
                            if let Some(id) = rows.get(x as usize) {
                                if let Some(tile) = data.game.tiles.get(id) {
                                    match &tile.kind {
                                        TileKind::TileSheet(id, tile_x, tile_y) => {
                                            if let Some(sheet) = data.game.tile_sheets.get(id) {
                                                let image =
                                                    data.image_caches.get(ctx, &sheet.file_path)?;
                                                RenderingParam::default()
                                                    .dest(
                                                        character.position.map_id,
                                                        x as f32,
                                                        y as f32,
                                                    )
                                                    .rect(
                                                        (sheet.tile_width * tile_x) as f32,
                                                        (sheet.tile_height * tile_y) as f32,
                                                        sheet.tile_width as f32,
                                                        sheet.tile_height as f32,
                                                    )
                                                    .target_size(64 as f32, 64 as f32)
                                                    .draw_image(&data.game, image, ctx)?;
                                            }
                                        }
                                        TileKind::Image(s) => {
                                            let image = data.image_caches.get(ctx, s)?;
                                            RenderingParam::default()
                                                .dest(character.position.map_id, x as f32, y as f32)
                                                .target_size(64 as f32, 64 as f32)
                                                .draw_image(&data.game, image, ctx)?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for (_, t) in data.game.teleportations.iter() {
                let image = data.image_caches.get(ctx, "/images/teleportation.png")?;
                RenderingParam::default()
                    .position(&t.position)
                    .target_size(64 as f32, 64 as f32)
                    .draw_image(&data.game, image, ctx)?;
            }

            // Draw characters
            let current_id = {
                if let Some(t) = &data.game.turn_system.current_turn {
                    if let TurnKind::Character { character_id } = &t.kind {
                        Some(*character_id)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            for (id, character) in data.game.characters.iter() {
                if character.hidden {
                    continue;
                }
                let avatar = &data
                    .game
                    .character_sprites
                    .get(&character.character_sprite_id)
                    .unwrap()
                    .avatar;
                let image = data.image_caches.get(ctx, avatar)?;
                RenderingParam::default()
                    .dest(
                        character.position.map_id,
                        character.position.x as f32 + character.offset_x,
                        character.position.y as f32 + character.offset_y,
                    )
                    .target_size(64 as f32, 64 as f32)
                    .draw_image(&data.game, &image, ctx)?;

                if current_id == Some(*id) {
                    // Draw border
                    Rendering::draw_character_border(ctx, &character, &data.game)?;
                }
            }

            for (_, sprite_animation) in data.game.sprite_animations.iter() {
                match &sprite_animation.animation {
                    Animation::SpriteSheet(id, name) => {
                        let sprite_sheet = data.game.sprite_sheets.get(&id).unwrap();
                        let image = data.image_caches.get(ctx, &sprite_sheet.image_path)?;
                        let frames = sprite_sheet.animation.get(name).unwrap();
                        let i = (frames.len() as f32 * sprite_animation.percentage) as usize;
                        let frame_index = if i >= frames.len() {
                            frames[frames.len() - 1]
                        } else {
                            frames[i]
                        };
                        RenderingParam::default()
                            .position(&sprite_animation.position)
                            .rect(
                                ((frame_index % sprite_sheet.frame_per_row)
                                    * sprite_sheet.frame_width)
                                    as f32,
                                ((frame_index / sprite_sheet.frame_per_row)
                                    * sprite_sheet.frame_height)
                                    as f32,
                                sprite_sheet.frame_width as f32,
                                sprite_sheet.frame_height as f32,
                            )
                            .target_size(64 as f32, 64 as f32)
                            .draw_image(&data.game, image, ctx)?;
                    }
                    Animation::SpriteSequence(id) => {
                        let sprite_sequence = data.game.sprite_sequences.get(&id).unwrap();
                        let i = (sprite_sequence.images.len() as f32 * sprite_animation.percentage)
                            as usize;
                        let path = if i >= sprite_sequence.images.len() {
                            &sprite_sequence.images[sprite_sequence.images.len() - 1]
                        } else {
                            &sprite_sequence.images[i]
                        };

                        let image = data.image_caches.get(ctx, path)?;
                        RenderingParam::default()
                            .position(&sprite_animation.position)
                            .target_size(64 as f32, 64 as f32)
                            .draw_image(&data.game, image, ctx)?;
                    }
                }
            }

            Rendering::draw_border(ctx, &data.game)?;

            // Draw ui, show characer hp, mp, etc...
            let stats_ui = Stats::new(ctx, character)?;
            stats_ui.draw_canvas(ctx)?;
            graphics::draw(
                ctx,
                &stats_ui.canvas,
                graphics::DrawParam {
                    dest: ggez::mint::Point2 { x: 720.0, y: 0.0 },
                    ..Default::default()
                },
            )?;
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
        if data.game.turn_system.waiting_input {
            match keycode {
                KeyCode::W => {
                    // Up
                    if let Some(t) = &data.game.turn_system.current_turn {
                        if let TurnKind::Character { character_id } = &t.kind {
                            let character_id = *character_id;
                            if let Ok(command) =
                                Character::move_command(&mut data.game, character_id, 0, -1)
                            {
                                data.game.add_player_command(character_id, command);
                            }
                        }
                    }
                }
                KeyCode::A => {
                    // Left
                    if let Some(t) = &data.game.turn_system.current_turn {
                        if let TurnKind::Character { character_id } = &t.kind {
                            let character_id = *character_id;
                            if let Ok(command) =
                                Character::move_command(&mut data.game, character_id, -1, 0)
                            {
                                data.game.add_player_command(character_id, command);
                            }
                        }
                    }
                }
                KeyCode::S => {
                    // Down
                    if let Some(t) = &data.game.turn_system.current_turn {
                        if let TurnKind::Character { character_id } = &t.kind {
                            let character_id = *character_id;
                            if let Ok(command) =
                                Character::move_command(&mut data.game, character_id, 0, 1)
                            {
                                data.game.add_player_command(character_id, command);
                            }
                        }
                    }
                }
                KeyCode::D => {
                    // Right
                    if let Some(t) = &data.game.turn_system.current_turn {
                        if let TurnKind::Character { character_id } = &t.kind {
                            let character_id = *character_id;
                            if let Ok(command) =
                                Character::move_command(&mut data.game, character_id, 1, 0)
                            {
                                data.game.add_player_command(character_id, command);
                            }
                        }
                    }
                }
                KeyCode::Q => {
                    // Cancel
                }
                KeyCode::E => {
                    // Confirm
                    if let Some(t) = &data.game.turn_system.current_turn {
                        if let TurnKind::Character { character_id } = &t.kind {
                            return Transition::Push(Box::new(ActionScene::new(
                                ctx,
                                &data,
                                *character_id,
                            )));
                        }
                    }
                }
                KeyCode::Escape => {
                    // Player can open pause menu anytime
                    return Transition::Push(Box::new(PauseScene::new(ctx, &data)));
                }
                _ => {}
            }
        } else {
            match keycode {
                KeyCode::Escape => {
                    // Player can open pause menu anytime
                    return Transition::Push(Box::new(PauseScene::new(ctx, &data)));
                }
                _ => {}
            }
        }

        Transition::None
    }
}
