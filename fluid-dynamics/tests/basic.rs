#[cfg(test)]
mod basic_flow {
    use mint::{Point2};

    use fluid_dynamics::{FluidGrid};
    use fluid_dynamics::{ConnectionGrid};

    struct Grid {
        grid: Vec<u8>,
        w: usize,
        h: usize
    }

    impl Grid {
        fn new(w: usize, h: usize) -> Self {
            Grid {
                grid: vec![0u8; w * h],
                w,
                h
            }
        }

        fn from_str(w: usize, h: usize, data: &str) -> Self {
            let mut grid = vec![];
            data.chars().for_each(|c| {
                match c {
                    '0' => grid.push(0),
                    '1' => grid.push(1),
                    _ => {}
                }
            });
            Grid {
                grid: grid,
                w,
                h
            }
        }

        fn index<T: Into<Point2<usize>>>(&self, point: T) -> usize {
            let point = point.into();
            if point.x >= self.w || point.y >= self.h {
                (self.w * self.h) + 1
            } else {
                point.x + (point.y * self.w)
            }
        }
    }

    impl ConnectionGrid for Grid {
        fn get_connections<T: Into<Point2<usize>>>(&self, pos: T) -> Vec<Point2<usize>> {
            let point = pos.into();
            let mut connections = vec![];
            if point.x != 0 {
                let nx = (point.x - 1) as usize;
                let ny = point.y as usize;
                if self.grid.get(self.index([nx, ny])) == Some(&0u8) {
                    connections.push([point.x - 1, point.y].into());
                }
            }
            if point.y != 0 {
                let nx = point.x as usize;
                let ny = (point.y - 1) as usize;
                if self.grid.get(self.index([nx, ny])) == Some(&0u8) {
                    connections.push([point.x, point.y - 1].into());
                }
            }
            if point.x < 4 {
                let nx = (point.x + 1) as usize;
                let ny = point.y as usize;
                if self.grid.get(self.index([nx, ny])) == Some(&0u8) {
                    connections.push([point.x + 1, point.y].into());
                }
            }
            if point.y < 4 {
                let nx = point.x as usize;
                let ny = (point.y + 1) as usize;
                if self.grid.get(self.index([nx, ny])) == Some(&0u8) {
                    connections.push([point.x, point.y + 1].into());
                }
            }
            connections
        }

        fn is_solid<T: Into<Point2<usize>>>(&self, pos: T) -> bool {
            let pos = pos.into();
            if pos.x >= self.w || pos.y >= self.h {
                return true;
            }
            let idx = self.index(pos);
            self.grid.get(idx) == Some(&1u8) || self.grid.get(idx) == None
        }
    }

    fn assert_fluid_eq(fluid: f32, target: f32) {
        println!("Comparing fluid level: {:.5} - {:5}", fluid, target);
        assert!((fluid - target).abs() < 0.001);
    }

    #[test]
    fn basic_setting_and_getting() {
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.get_fluid([0, 0]), Some(&0f32));
        assert_fluid_eq(grid.total_fluid_level(), 0f32);

        assert_eq!(grid.set_fluid([2, 2], 3f32), Some(3f32));
        assert_eq!(grid.get_fluid([2, 2]), Some(&3f32));
        assert_fluid_eq(grid.total_fluid_level(), 3f32);

        assert_eq!(grid.set_fluid([20, 2], 3f32), None);

        assert_eq!(grid.set_fluid([3, 3], 10f32), Some(10f32));
        assert_eq!(grid.get_fluid([3, 3]), Some(&10f32));

        assert_eq!(grid.add_fluid([3, 3], 10f32), Some(20f32));
        assert_eq!(grid.get_fluid([3, 3]), Some(&20f32));
    }

    #[test]
    fn flow_from_corner() {
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.set_fluid([0, 0], 10f32), Some(10f32));
        assert_fluid_eq(grid.total_fluid_level(), 10f32);

        let g = Grid::new(5, 5);

        grid.flow(&g);

        assert_eq!(grid.get_fluid([1, 0]), Some(&2.5f32));
        assert_eq!(grid.get_fluid([0, 1]), Some(&2.5f32));
        assert_eq!(grid.get_fluid([0, 0]), Some(&5f32));
        assert_fluid_eq(grid.total_fluid_level(), 10f32);
    }

    #[test]
    fn flow_in_center() {
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.set_fluid([2, 2], 10f32), Some(10f32));
        assert_fluid_eq(grid.total_fluid_level(), 10f32);

        let g = Grid::new(5, 5);

        grid.flow(&g);

        assert_eq!(grid.get_fluid([2, 2]), Some(&0f32));
        assert_eq!(grid.get_fluid([1, 2]), Some(&2.5f32));
        assert_eq!(grid.get_fluid([3, 2]), Some(&2.5f32));
        assert_eq!(grid.get_fluid([2, 1]), Some(&2.5f32));
        assert_eq!(grid.get_fluid([2, 3]), Some(&2.5f32));
        assert_fluid_eq(grid.total_fluid_level(), 10f32);
    }

    #[test]
    fn flow_between_two() {
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.set_fluid([0, 0], 10f32), Some(10f32));
        assert_eq!(grid.set_fluid([1, 0], 20f32), Some(20f32));
        assert_fluid_eq(grid.total_fluid_level(), 30f32);

        let g = Grid::new(5, 5);

        grid.flow(&g);

        assert_eq!(grid.get_fluid([0, 0]), Some(&10f32)); 
        assert_eq!(grid.get_fluid([0, 1]), Some(&2.5f32));
        assert_eq!(grid.get_fluid([1, 0]), Some(&7.5f32)); 
        assert_eq!(grid.get_fluid([1, 1]), Some(&5f32));
        assert_eq!(grid.get_fluid([2, 0]), Some(&5f32));
        assert_fluid_eq(grid.total_fluid_level(), 30f32);
    }

    #[test]
    fn constant_fluid_level() {
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.set_fluid([1, 0], 21f32), Some(21f32));
        assert_fluid_eq(grid.total_fluid_level(), 21f32);

        let g = Grid::new(5, 5);

        grid.flow(&g);
        assert_fluid_eq(grid.total_fluid_level(), 21f32);

        grid.flow(&g);
        assert_fluid_eq(grid.total_fluid_level(), 21f32);

        grid.flow(&g);
        assert_fluid_eq(grid.total_fluid_level(), 21f32);

        grid.flow(&g);
        assert_fluid_eq(grid.total_fluid_level(), 21f32);
    }

    #[test]
    fn level_equalizer() {
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.set_fluid([0, 0], 50f32), Some(50f32));

        let g = Grid::new(5, 5);

        for _ in 0..500 {
            grid.flow(&g);
        }
        assert!(grid.is_stable());
        assert_fluid_eq(grid.total_fluid_level(), 50f32);
    }

    #[test]
    fn obstacle_flow() {
        let mut grid = FluidGrid::new(5, 5);
        let g = Grid::from_str(5, 5, "
            00000
            00000
            11011
            00010
            00000
        ");

        assert_eq!(grid.set_fluid([0, 0], 50f32), Some(50f32));

        for _ in 0..500 {
            grid.flow(&g);
        }

        assert_fluid_eq(grid.total_fluid_level(), 50f32);
        assert!(grid.is_stable());
        assert_eq!(grid.get_fluid([0, 2]), Some(&0f32));
        assert_eq!(grid.get_fluid([1, 2]), Some(&0f32));
        assert_eq!(grid.get_fluid([3, 2]), Some(&0f32));
        assert_eq!(grid.get_fluid([4, 2]), Some(&0f32));
        assert_eq!(grid.get_fluid([3, 3]), Some(&0f32));
        assert!(*grid.get_fluid([4, 3]).unwrap() > 2.4f32);
        assert!(*grid.get_fluid([2, 2]).unwrap() > 2.4f32);
   }

    #[test]
    fn basic_pressure() {
        let g = Grid::from_str(5, 5, "
            00000
            00000
            11011
            00010
            00000
        ");
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.set_fluid([0, 0], 50f32), Some(50f32));

        for _ in 0..500 {
            grid.flow(&g);
        }
        assert!(grid.is_stable());
        assert_fluid_eq(grid.total_fluid_level(), 50f32);
        assert_fluid_eq(*grid.get_pressure([0, 0]).unwrap(), 0f32);

        assert!(*grid.get_pressure([0, 2]).unwrap() > 5f32);
        assert!(*grid.get_pressure([0, 2]).unwrap() < 6f32);

        assert!(*grid.get_pressure([3, 3]).unwrap() > 7f32);
        assert!(*grid.get_pressure([3, 3]).unwrap() < 8f32);
   }

    #[test]
    fn basic_velocity_flow() {
        let g = Grid::from_str(5, 5, "
            00000
            11011
            01010
            00000
            00000
        ");
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.set_fluid([2, 0], 60f32), Some(60f32));
        grid.flow(&g);

        println!("{:?}", grid);
        assert_fluid_eq(*grid.get_fluid([2, 0]).unwrap(), 15f32);
        assert_fluid_eq(*grid.get_fluid([2, 1]).unwrap(), 15f32);
        assert_fluid_eq(*grid.get_fluid([2, 2]).unwrap(), 0f32);

        grid.flow(&g);
        assert!(*grid.get_fluid([2, 0]).unwrap() >= 15f32);
        assert!(*grid.get_fluid([2, 1]).unwrap() > 5f32);
        assert!(*grid.get_fluid([2, 2]).unwrap() > 0f32);
        assert_fluid_eq(*grid.get_fluid([2, 3]).unwrap(), 0f32);

        grid.flow(&g);
        assert!(*grid.get_fluid([2, 0]).unwrap() >= 10f32);
        assert!(*grid.get_fluid([2, 1]).unwrap() > 10f32);
        assert!(*grid.get_fluid([2, 2]).unwrap() > 4f32);
        assert!(*grid.get_fluid([2, 3]).unwrap() > 0f32);
        assert_fluid_eq(*grid.get_fluid([2, 4]).unwrap(), 0f32);

        grid.flow(&g);
        assert!(*grid.get_fluid([2, 0]).unwrap() >= 10f32);
        assert!(*grid.get_fluid([2, 1]).unwrap() > 9f32);
        assert!(*grid.get_fluid([2, 2]).unwrap() > 4f32);
        assert!(*grid.get_fluid([2, 3]).unwrap() > 1f32);

        assert!(*grid.get_fluid([2, 4]).unwrap() > 0f32);
        assert!(*grid.get_fluid([3, 3]).unwrap() > 0f32);
        assert!(*grid.get_fluid([1, 3]).unwrap() > 0f32);

        assert!(*grid.get_fluid([2, 4]).unwrap() > *grid.get_fluid([1, 3]).unwrap());
   }

    #[test]
    fn constant_fluid_after_adding() {
        let mut grid = FluidGrid::new(5, 5);

        assert_eq!(grid.set_fluid([2, 2], 100f32), Some(100f32));
        assert_fluid_eq(grid.total_fluid_level(), 100f32);

        let g = Grid::from_str(5, 5, "
            11111
            10001
            10001
            10001
            11111
        ");

        for _ in 0..100 {
            grid.flow(&g);
        }
        assert_fluid_eq(grid.total_fluid_level(), 100f32);

        assert_fluid_eq(*grid.get_fluid([2, 2]).unwrap(), 11.1111f32);
        grid.add_fluid([2, 2], 100f32);
        assert_fluid_eq(*grid.get_fluid([2, 2]).unwrap(), 111.1111f32);
        assert_fluid_eq(grid.total_fluid_level(), 200f32);

        for _ in 0..100 {
            grid.flow(&g);
        }

        assert_fluid_eq(grid.total_fluid_level(), 200f32);
    }
}
