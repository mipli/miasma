use mint::Point2;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Wall
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Floor => '.',
            Tile::Wall => '#',
        }
    }
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            tiles: vec![Tile::Wall; width * height],
            width,
            height,
        }
    }

    pub fn iter(&self) -> MapIterator {
        MapIterator::new(&self)
    }

    pub fn index<T: Into<Point2<usize>>>(&self, point: T) -> usize {
        let point = point.into();
        if point.x >= self.width || point.y >= self.height {
            (self.width * self.height) + 1
        } else {
            point.x + (point.y * self.width)
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Map, ()> {
        let mut height = 0;
        let tiles = input.chars().filter_map(|c| {
            match c {
                '.' => Some(Tile::Floor),
                '#' => Some(Tile::Wall),
                '\n' => {
                    height += 1;
                    None
                }
                _ => None
            }
        }).collect::<Vec<_>>();
        let width = tiles.len() / height;
        Ok(Map {
            tiles,
            width,
            height
        })
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = String::new();
        let mut last_y = 0;
        for (i, tile) in self.tiles.iter().enumerate() {
            let (x, y) = (i % self.width, i / self.width);
            if y > last_y {
                buf.push('\n');
            }
            last_y = y;
            let c: char = (*tile).into();
            buf.push(c);
        }

        write!(f, "{}", buf)
    }
}

pub struct MapIterator<'a> {
    map: &'a Map,
    next: usize
}

impl<'a> MapIterator<'a> {
    fn new(map: &'a Map) -> Self {
        MapIterator {
            map,
            next: 0
        }
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = (Point2<usize>, Tile);

    fn next(&mut self) -> Option<(Point2<usize>, Tile)> {
        let (x, y) = (self.next % self.map.width, self.next / self.map.width);
        let tile = self.map.tiles.get(self.next)?;
        self.next += 1;
        Some(([x, y].into(), *tile))
    }
}
