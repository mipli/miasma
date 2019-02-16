use mint::{Point2};

pub trait ConnectionGrid {
    fn get_connections<T: Into<Point2<usize>>>(&self, pos: T) -> Vec<Point2<usize>>;
    fn is_solid<T: Into<Point2<usize>>>(&self, pos: T) -> bool;
}
