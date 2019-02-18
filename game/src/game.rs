use mint::Point2;
use fluid_dynamics::FluidGrid;

use fluid_dynamics::ConnectionGrid;
use crate::map::{Tile, Map};
use crate::console::{Console, Cell};
use crate::entities::{EntityID, EntityManager, Visual, Physics};

pub struct World {
    pub map: Map,
    pub entity_manager: EntityManager,
}

impl World {
    fn new(map: Map) -> Self {
        World {
            map,
            entity_manager: EntityManager::default(),
        }
    }
}

pub struct GameState {
    pub pos: Point2<usize>,
    pub miasma: FluidGrid,
    pub world: World
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
            world: World::new(map),
        }
    }

    pub fn draw(&self, console: &mut Console) {
        self.blit_map(console);
        self.blit_miasma(console);
        self.blit_entities(console);
    }

    pub fn flow(&mut self) {
        for _ in 0..5 {
            self.miasma.flow(&self.world);
        }
    }

    pub fn handle_pressure(&mut self) {
        let (miasma, world) = (&self.miasma, &mut self.world);
        let mut to_delete = vec![];
        world.entity_manager.physics.iter_mut().for_each(|(id, physics)| {
            if physics.durability > 0 {
                if let Some(pressure) = miasma.get_pressure(physics.position) {
                    let pressure = pressure.floor() as u32;
                    if pressure >= physics.hardness {
                        if pressure <= physics.durability {
                            physics.durability = physics.durability - pressure;
                        } else {
                            physics.durability = 0;
                        }
                        if physics.durability == 0 {
                            to_delete.push(*id);
                        }
                    }
                }
            }
        });
        to_delete.iter().for_each(|id| {
            world.entity_manager.delete_entity(id);
        });
    }

    fn blit_map(&self, console: &mut Console) {
        for (pos, tile) in self.world.map.iter() {
            let c: char = tile.into();
            console.set(pos, Cell {
                glyph: c,
                ..Cell::default()
            });
        }
    }

    pub fn add_door(&mut self, pos: Point2<usize>) {
        let entity = self.world.entity_manager.create_entity();
        self.world.entity_manager.add_visual(entity, Visual {
            glyph: '=',
            foreground: (1f32, 0f32, 1f32, 1f32).into()
        });
        self.world.entity_manager.add_physics(entity, Physics {
            position: pos,
            hardness: 5,
            durability: 100
        });
    }

    fn blit_entities(&self, console: &mut Console) {
        console.set(self.pos, Cell {
            glyph: '@',
            foreground: (0.7, 0.0, 0.0, 1.0).into(),
            ..Cell::default()
        });
        self.world.entity_manager.visual.iter().for_each(|(id, visual)| {
            if let Some(physics) = self.world.entity_manager.get_physics(id) {
                console.set(physics.position, Cell {
                    glyph: visual.glyph,
                    foreground: visual.foreground,
                    ..Cell::default()
                });
            }
        });
    }

    fn blit_miasma(&self, console: &mut Console) {
        for (pos, fluid) in self.miasma.iter() {
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
                console.set(pos, Cell {
                    glyph,
                    foreground: (0.0, 0.4, 0.3, 1.0).into(),
                    ..Cell::default()
                });
            }
        }
    }
}

impl ConnectionGrid for World {
    fn get_connections<T: Into<Point2<usize>>>(&self, pos: T) -> Vec<Point2<usize>> {
        let point = pos.into();
        let mut connections = vec![];
        if point.x != 0 {
            let nx = (point.x - 1) as usize;
            let ny = point.y as usize;
            if !self.is_solid([nx, ny]) {
                connections.push([point.x - 1, point.y].into());
            }
        }
        if point.y != 0 {
            let nx = point.x as usize;
            let ny = (point.y - 1) as usize;
            if !self.is_solid([nx, ny]) {
                connections.push([point.x, point.y - 1].into());
            }
        }
        if point.x < self.map.width - 1 {
            let nx = (point.x + 1) as usize;
            let ny = point.y as usize;
            if !self.is_solid([nx, ny]) {
                connections.push([point.x + 1, point.y].into());
            }
        }
        if point.y < self.map.height - 1 {
            let nx = point.x as usize;
            let ny = (point.y + 1) as usize;
            if !self.is_solid([nx, ny]) {
                connections.push([point.x, point.y + 1].into());
            }
        }
        connections
    }

    fn is_solid<T: Into<Point2<usize>>>(&self, pos: T) -> bool {
        let pos = pos.into();
        if pos.x >= self.map.width || pos.y >= self.map.height {
            return true;
        }
        let idx = self.map.index(pos);
        let has_entity = self.entity_manager.physics.iter().any(|(_, physics)| {
            physics.position == pos
        });
        has_entity || self.map.tiles.get(idx) == Some(&Tile::Wall) || self.map.tiles.get(idx) == None
    }
}
