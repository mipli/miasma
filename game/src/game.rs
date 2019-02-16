use mint::Point2;
use fluid_dynamics::FluidGrid;

use crate::map::Map;

pub struct GameState {
    pub pos: Point2<usize>,
    pub map: Map,
    pub miasma: FluidGrid
}

impl GameState {
    /*
    pub fn new(width: usize, height: usize) -> Self {
        GameState {
            pos: [5, 5].into(),
            map: Map::new(width, height),
            miasma: FluidGrid::new(width, height),
        }
    }
    */

    pub fn from_str(input: &str) -> Self {
        let map: Map = input.parse().unwrap();
        GameState {
            pos: [5, 5].into(),
            miasma: FluidGrid::new(map.width, map.height),
            map,
        }
    }

    pub fn flow(&mut self) {
        self.miasma.flow(&self.map);
    }
}

