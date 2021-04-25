use data::Data;
use ggez::event::{self, EventHandler};
use ggez::{conf, event::KeyCode};
use ggez::{event::KeyMods, filesystem};
use ggez::{Context, ContextBuilder, GameResult};
use scene::SceneStack;
use scenes::{action_scene::ActionScene, level_scene::LevelScene};
use std::{env, path};
use tower::core::Game;

mod data;
mod rendering;
mod scene;
mod scenes;
mod ui;

fn setup_logger() -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};
    let colors = ColoredLevelConfig::default()
        .info(Color::Green)
        .debug(Color::BrightMagenta)
        .trace(Color::BrightBlue);
    // This sets up a `fern` logger and initializes `log`.
    fern::Dispatch::new()
        // Formats logs
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{:<5}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("tower", log::LevelFilter::Trace)
        .level_for("gfx", log::LevelFilter::Warn)
        .level_for("gfx_device_gl", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()
        .expect("Could not init logging!");

    // Test logger
    // log::trace!("Trace");
    // log::debug!("Debug");
    // log::info!("Info");
    // log::warn!("Warn");
    // log::error!("Error");

    Ok(())
}

fn main() {
    setup_logger().unwrap();

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("tower", "ueshima")
        .window_setup(conf::WindowSetup::default().title("The tower"))
        .window_mode(conf::WindowMode::default().dimensions(1280.0, 720.0))
        .add_resource_path(&resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MainState::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

pub struct MainState {
    // Your state here...\
    pub scene_stack: SceneStack,
    pub data: data::Data,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        // Load/create resources such as images here.
        // filesystem::create_dir(ctx, "/maps").unwrap();
        // let mut data = data::Data::new();
        // let f = filesystem::create(ctx, "/data.ron").unwrap();
        // let pretty = ron::ser::PrettyConfig::new();
        // ron::ser::to_writer_pretty(f, &data, pretty).unwrap();

        let characters = filesystem::open(ctx, "/game/characters.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let commands = filesystem::open(ctx, "/game/commands.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let character_actions = filesystem::open(ctx, "/game/character_actions.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let actions = filesystem::open(ctx, "/game/actions.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let items = filesystem::open(ctx, "/game/items.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let tile_maps = filesystem::open(ctx, "/game/tile_maps.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let teleportations = filesystem::open(ctx, "/game/teleportations.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let tiles = filesystem::open(ctx, "/game/tiles.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let tile_sheets = filesystem::open(ctx, "/game/tile_sheets.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let character_sprites = filesystem::open(ctx, "/game/character_sprites.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let sprite_sheets = filesystem::open(ctx, "/game/sprite_sheets.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let sprite_sequences = filesystem::open(ctx, "/game/sprite_sequences.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let sprite_animations = filesystem::open(ctx, "/game/sprite_animations.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let areas = filesystem::open(ctx, "/game/areas.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let camera = filesystem::open(ctx, "/game/camera.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let turn_system = filesystem::open(ctx, "/game/turn_system.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let novel_system = filesystem::open(ctx, "/game/novel_system.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let shop_system = filesystem::open(ctx, "/game/shop_system.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let floor_system = filesystem::open(ctx, "/game/floor_system.ron")
            .map(|f| ron::de::from_reader(f).unwrap())
            .unwrap();
        let game = Game {
            characters,
            commands,
            character_actions,
            actions,
            items,
            tile_maps,
            teleportations,
            tiles,
            tile_sheets,
            character_sprites,
            sprite_sheets,
            sprite_sequences,
            sprite_animations,
            areas,
            camera,
            turn_system,
            novel_system,
            shop_system,
            floor_system,
        };
        // let f = filesystem::open(ctx, "/game.ron").unwrap();
        // let game: Game = ron::de::from_reader(f).unwrap();
        let mut data = Data::new(game);
        let mut scene_stack = SceneStack::new();
        scene_stack
            .stack
            .push(Box::new(LevelScene::new(ctx, &mut data)));
        MainState { scene_stack, data }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scene_stack.update(ctx, &mut self.data)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scene_stack.draw(ctx, &mut self.data)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        self.scene_stack
            .key_down_event(ctx, keycode, keymods, repeat, &mut self.data);
    }
}
