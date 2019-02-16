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

use console::{Console, Cell};
use game::GameState;

struct GameScreen {
    console: Console,
    state: GameState
}

impl GameScreen {
    fn blit_map(&mut self) {
        for (pos, tile) in self.state.map.iter() {
            let c: char = tile.into();
            self.console.set(pos, Cell {
                glyph: c,
                ..Cell::default()
            });
        }
    }

    fn blit_entities(&mut self) {
        self.console.set(self.state.pos, Cell {
            glyph: '@',
            foreground: (0.7, 0.0, 0.0, 1.0).into(),
            ..Cell::default()
        });
    }

    fn blit_miasma(&mut self) {
        for (pos, fluid) in self.state.miasma.iter() {
            if fluid > 0f32 {
                let glyph = match fluid {
                    f if f >= 9f32 => '9',
                    f if f >= 8f32 => '8',
                    f if f >= 7f32 => '7',
                    f if f >= 6f32 => '6',
                    f if f >= 5f32 => '5',
                    f if f >= 4f32 => '4',
                    f if f >= 3f32 => '3',
                    f if f >= 2f32 => '2',
                    f if f >= 1f32 => '1',
                    _ => '0'
                };
                self.console.set(pos, Cell {
                    glyph,
                    foreground: (0.0, 0.4, 0.3, 1.0).into(),
                    ..Cell::default()
                });
            }
        }
    }
}

impl State for GameScreen {
    fn new() -> Result<Self> {
        let map_contents: String = load_file("map").map(move |bytes| {
            bytes.into_iter().map(|c| c as char).collect()
        }).wait()?;
        let state = GameState::from_str(&map_contents);
        Ok(GameScreen {
            console: Console::new(state.map.width, state.map.height, "square.ttf".to_string()),
            state
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        use quicksilver::input::ButtonState::*;

        if window.keyboard()[Key::H] == Pressed {
            self.state.pos = [self.state.pos.x - 1, self.state.pos.y].into()
        }
        if window.keyboard()[Key::K] == Pressed {
            self.state.pos = [self.state.pos.x, self.state.pos.y - 1].into()
        }
        if window.keyboard()[Key::J] == Pressed {
            self.state.pos = [self.state.pos.x, self.state.pos.y + 1].into()
        }
        if window.keyboard()[Key::L] == Pressed {
            self.state.pos = [self.state.pos.x + 1, self.state.pos.y].into()
        }
        if window.keyboard()[Key::Period] == Pressed {
            println!("Inserting fluid");
            self.state.miasma.add_fluid(self.state.pos, 100f32);
            let fluid = self.state.miasma.total_fluid_level();
            println!("Fluid level: {}", fluid);
        }
        if window.keyboard()[Key::V] == Pressed {
            let fluid = self.state.miasma.total_fluid_level();
            println!("Fluid level: {}", fluid);
        }

        if window.keyboard()[Key::Q].is_down() {
            window.close();
        }
        if window.keyboard()[Key::Escape].is_down() {
            window.close();
        }

        self.state.flow();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.blit_map();
        self.blit_miasma();
        self.blit_entities();

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
