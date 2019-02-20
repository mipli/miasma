use mint::{Point2, Vector2};

use crate::{ConnectionGrid};

pub struct FluidGrid {
    width: usize,
    height: usize,
    viscocity: f32,
    fluid: Vec<f32>,
    pressure: Vec<f32>,
    velocity: Vec<Vector2<i32>>
}

impl FluidGrid {
    pub fn new(width: usize, height: usize) -> FluidGrid {
        FluidGrid {
            width,
            height,
            viscocity: 1f32,
            fluid: vec![0f32; width * height],
            pressure: vec![0f32; width * height],
            velocity: vec![Vector2 { x: 0i32, y: 0i32}; width * height],
        }
    }

    /*
     * Setting viscocity higher than 1.0 does not really make sense physcially
     */
    pub fn set_viscocity(&mut self, viscocity: f32) {
        self.viscocity = viscocity;
    }

    pub fn dimensions(&self) -> Vector2<usize> {
        [self.width, self.height].into()
    }

    fn index(&self, point: &Point2<usize>) -> usize {
        point.x + (point.y * self.width)
    }

    fn valid_position(&self, point: &Point2<usize>) -> bool {
        point.x < self.width && point.y < self.height
    }

    pub fn get_fluid<T: Into<Point2<usize>>>(&self, point: T) -> Option<&f32> {
        let point = point.into();
        if !self.valid_position(&point) {
            None
        } else {
            let index = self.index(&point);
            self.fluid.get(index)
        }
    }

    pub fn get_pressure<T: Into<Point2<usize>>>(&self, point: T) -> Option<&f32> {
        let point = point.into();
        if !self.valid_position(&point) {
            None
        } else {
            let index = self.index(&point);
            self.pressure.get(index)
        }
    }

    pub fn get_velocity<T: Into<Point2<usize>>>(&self, point: T) -> Option<&Vector2<i32>> {
        let point = point.into();
        if !self.valid_position(&point) {
            None
        } else {
            let index = self.index(&point);
            self.velocity.get(index)
        }
    }

    pub fn add_fluid<T: Into<Point2<usize>>>(&mut self, point: T, value: f32) -> Option<f32> {
        let point = point.into();
        let index = self.index(&point);
        if let Some(fluid) = self.fluid.get(index) {
            println!("adding fluid: {:.4} + {}", fluid, value);
            let n = fluid + value;
            println!("new: {:.4}", n);
            self.fluid[index] = n;
            Some(n)
        } else {
            None
        }
    }

    pub fn set_fluid<T: Into<Point2<usize>>>(&mut self, point: T, value: f32) -> Option<f32> {
        let point = point.into();
        let index = self.index(&point);
        if index <= self.fluid.len() {
            self.fluid.insert(index, value);
            Some(value)
        } else {
            None
        }
    }

    pub fn total_fluid_level(&self) -> f32 {
        self.fluid
            .iter()
            .sum()
    }

    pub fn is_stable(&self) -> bool {
        let base = self.fluid[0];
        let delta = 0.01;
        self.fluid
            .iter()
            .all(|f| *f == 0f32 || (base - *f).abs() < delta)
    }

    pub fn flow(&mut self, connection_grid: &impl ConnectionGrid) {
        let (fluid, pressure, velocity) = self.calculate_flow(connection_grid);
        self.fluid = fluid;
        self.pressure = pressure;
        self.velocity = velocity;
    }

    fn calculate_flow(&self, connection_grid: &impl ConnectionGrid) -> (Vec<f32>, Vec<f32>, Vec<Vector2<i32>>) {
        let mut fluid = vec![0f32; self.width * self.height];
        let mut pressure = vec![0f32; self.width * self.height];
        let mut velocity = vec![Vector2 { x: 0i32, y: 0i32}; self.width * self.height];

        for x in 0..self.width {
            for y in 0..self.height {
                let point: Point2<usize> = [x, y].into();
                let idx = self.index(&point);
                let connections = connection_grid.get_connections([x, y]);
                let flow_connections = connections
                    .iter()
                    .filter_map(|pos| {
                        match self.get_fluid(*pos) {
                            Some(f) if *f > 0f32 => {
                                let i = self.index(&pos);
                                let v = self.velocity[i];
                                let vx: Point2<i32> = [pos.x as i32 + v.x, pos.y as i32 + v.y].into();
                                let connection_count = connection_grid.get_connections([pos.x, pos.y]).len();
                                let flow = f.clone() / 4f32;
                                if v.x == 0i32 && v.y == 0i32 || connection_count != 4 {
                                    Some((pos.clone(), flow))
                                } else if vx.x == x as i32 && vx.y == y as i32 {
                                    Some((pos.clone(), 4f32 * 5f32 * (flow / 8f32)))
                                } else {
                                    Some((pos.clone(), 4f32 * (flow / 8f32)))
                                }
                            },
                            _ => None
                        }
                    })
                    .collect::<Vec<_>>();
                let in_flow: f32 = flow_connections
                    .iter()
                    .map(|(_, f)| *f)
                    .sum();
                if connection_grid.is_solid([x, y]) {
                    pressure[idx] = in_flow * 4f32;
                } else {
                    pressure[idx] = 0f32;
                    let out_flow: f32 = self.fluid[idx] / 4f32;
                    let out_flow = out_flow * connections.len() as f32;
                    fluid[idx] = self.fluid[idx] + self.viscocity * (in_flow - out_flow);

                    if flow_connections.len() == 1 {
                        let dx = x as i32 - flow_connections[0].0.x as i32;
                        let dy = y as i32 - flow_connections[0].0.y as i32;
                        velocity[idx] = Vector2 {
                            x: dx,
                            y: dy
                        };
                    } else {
                        velocity[idx] = Vector2 {
                            x: 0,
                            y: 0
                        };
                    }
                }
            }
        }
        (fluid, pressure, velocity)
    }

    pub fn print_velocity(&self) {
        let mut buf = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(v) = self.get_velocity([x, y]) {
                    buf.push_str(&format!(" {:02},{:02} ", v.x, v.y));
                }
            }
            buf.push('\n');
        }
        println!("{}\n\n", buf)
    }

    pub fn iter(&self) -> FluidIterator {
        FluidIterator::new(&self)
    }
}

impl std::fmt::Debug for FluidGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(v) = self.get_fluid([x, y]) {
                    buf.push_str(&format!(" {:06.3} ", v));
                }
            }
            buf.push('\n');
        }
        writeln!(f, "{}", buf)
    }
}

pub struct FluidIterator<'a> {
    fluid_grid: &'a FluidGrid,
    next: usize
}

impl<'a> FluidIterator<'a> {
    fn new(fluid_grid: &'a FluidGrid) -> Self {
        FluidIterator {
            fluid_grid,
            next: 0
        }
    }
}

impl<'a> Iterator for FluidIterator<'a> {
    type Item = (Point2<usize>, f32);

    fn next(&mut self) -> Option<(Point2<usize>, f32)> {
        let (x, y) = (self.next % self.fluid_grid.width, self.next / self.fluid_grid.width);
        let tile = self.fluid_grid.fluid.get(self.next)?;
        self.next += 1;
        Some(([x, y].into(), *tile))
    }
}

