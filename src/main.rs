use std::path;

use ggez::event::{KeyCode, KeyMods};
use ggez::{conf, event, timer, Context, GameResult};
use specs::{RunNow, World, WorldExt};

mod components;
mod constants;
mod levels;
mod resources;
mod systems;

use components::register_components;
use levels::initialize_level;
use resources::{register_resources, InputQueue, Time};
use systems::{GameplayStateSystem, InputSystem, RenderingSystem};

struct Game {
    world: World,
}

impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        // Get and update time resource
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(context);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}

fn main() -> GameResult {
    let mut world = World::new();

    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    let game = &mut Game { world };

    event::run(context, event_loop, game)
}
