use mint::{Point2};
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{
        Background::{Blended, Col, Img},
        Color as QuickColor, Font, FontStyle, Image,
    },
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Color {
            r, 
            b,
            g, 
            a
        }
    }
}

impl From<Color> for QuickColor {
    fn from(color: Color) -> Self {
        QuickColor {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub glyph: char,
    pub foreground: Color,
    pub background: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            glyph: '=',
            foreground: (1.0, 1.0, 1.0, 1.0).into(),
            background: (0.0, 0.0, 0.0, 1.0).into(),
        }
    }
}

pub struct Console {
    width: usize,
    height: usize,
    tileset: Asset<HashMap<char, Image>>,
    tile_size: Vector,
    cells: Vec<Cell>,
}

impl Console {
    pub fn new(width: usize, height: usize, font: String) -> Console {
        let tile_size = Vector::new(12, 12);
        let game_glyphs = (30u8..250u8).map(|c| {
            c as char
        }).collect::<String>();
        let tileset = Asset::new(Font::load(font).and_then(move |text| {
            let tiles = text
                .render(&game_glyphs, &FontStyle::new(12f32, QuickColor::WHITE))
                .expect("Could not render the font tileset.");
            let mut tileset = HashMap::new();
            println!("tile size: {:?}", tile_size);
            println!("tiles: {:?}", tiles.area());
            for (index, glyph) in game_glyphs.chars().enumerate() {
                let pos = (index as i32 * tile_size.x as i32, 0);
                let tile = tiles.subimage(Rectangle::new(pos, tile_size));
                tileset.insert(glyph, tile);
            }
            Ok(tileset)
        }));

        Console {
            width,
            height,
            cells: vec![Cell::default(); width * height],
            tileset,
            tile_size
        }
    }

    pub fn clear(&mut self) {
       self.cells.iter_mut().for_each(|cell| {
           *cell = Cell::default();
       });
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let width = self.width;
        let tile_size = self.tile_size;
        let (tileset, cells) = (&mut self.tileset, &self.cells);
        tileset.execute(|tileset| {
            for (i, cell) in cells.iter().enumerate() {
                let (x, y) = (i % width, i / width);
                if let Some(image) = tileset.get(&cell.glyph) {
                    let pos = Vector::new(x as f32, y as f32).times(tile_size);
                    window.draw(
                        &Rectangle::new(pos, image.area().size()),
                        Blended(&image, cell.foreground.into()),
                    );
                }
            }
            Ok(())
        })?;

        Ok(())
    }

    pub fn set<T: Into<Point2<usize>>>(&mut self, pos: T, cell: Cell) {
        let pos = pos.into();
        let index = self.get_index(pos);
        self.cells[index] = cell;
    }

    fn get_index(&self, pos: Point2<usize>) -> usize {
        pos.x + (pos.y * self.width)
    }
}
