use quicksilver::{
    load_file,
    geom::Vector,
    graphics::Color,
    input::Key,
    lifecycle::{run, Settings, State, Window},
    Future, Result,
};

mod console;
mod game;
mod map;
mod entities;

use console::{Console, Cell};
use game::GameState;

struct GameScreen {
    console: Console,
    state: GameState
}

impl GameScreen {
}

impl State for GameScreen {
    fn new() -> Result<Self> {
        let map_contents: String = load_file("map").map(move |bytes| {
            bytes.into_iter().map(|c| c as char).collect()
        }).wait()?;
        let state = GameState::from_str(&map_contents);
        Ok(GameScreen {
            console: Console::new(state.world.map.width, state.world.map.height, "square.ttf".to_string()),
            state
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        use quicksilver::input::ButtonState::*;

        let mut action = false;
        if window.keyboard()[Key::H] == Pressed {
            self.state.pos = [self.state.pos.x - 1, self.state.pos.y].into();
            action = true;
        }
        if window.keyboard()[Key::K] == Pressed {
            self.state.pos = [self.state.pos.x, self.state.pos.y - 1].into();
            action = true;
        }
        if window.keyboard()[Key::J] == Pressed {
            self.state.pos = [self.state.pos.x, self.state.pos.y + 1].into();
            action = true;
        }
        if window.keyboard()[Key::L] == Pressed {
            self.state.pos = [self.state.pos.x + 1, self.state.pos.y].into();
            action = true;
        }

        if window.keyboard()[Key::Period] == Pressed {
            action = true;
        }
        if window.keyboard()[Key::W] == Pressed {
            println!("Inserting fluid");
            self.state.miasma.add_fluid(self.state.pos, 100f32);
            let fluid = self.state.miasma.total_fluid_level();
            println!("Fluid level: {}", fluid);
        }
        if window.keyboard()[Key::V] == Pressed {
            let fluid = self.state.miasma.total_fluid_level();
            println!("Fluid level: {}", fluid);
        }
        if window.keyboard()[Key::D] == Pressed {
            self.state.add_door(self.state.pos);
        }
        if window.keyboard()[Key::P] == Pressed {
             let pressure = self.state.miasma.get_pressure(self.state.pos);
             println!("Pressure: {:?}", pressure);
        }
        if window.keyboard()[Key::X] == Pressed {
             if let Some((id, _)) = self.state.world.entity_manager.physics.iter().find(|(_, physics)| {
                 physics.position == self.state.pos
             }) {
                println!("physics: {:?}", self.state.world.entity_manager.get_physics(id));
             }
        }

        if window.keyboard()[Key::Q].is_down() {
            window.close();
        }
        if window.keyboard()[Key::Escape].is_down() {
            window.close();
        }

        if action {
            self.state.flow();
            self.state.handle_pressure();
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.state.draw(&mut self.console);
        window.clear(Color::BLACK)?;
        self.console.draw(window)?;

        self.console.clear();
        Ok(())
    }
}

pub fn main() -> Result<()> {
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<GameScreen>("Miasma", Vector::new(800, 600), settings);
    Ok(())
}
