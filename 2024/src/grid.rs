use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T = usize> {
    pub x: T,
    pub y: T,
}

impl Into<Point<i32>> for Point<usize> {
    fn into(self) -> Point<i32> {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl Into<Point<usize>> for Point<i32> {
    fn into(self) -> Point<usize> {
        Point {
            x: self.x as usize,
            y: self.y as usize,
        }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T> Grid<T> {
    pub fn from_str(s: &str, parse: fn(char) -> T) -> Grid<T> {
        let mut grid = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in s.lines() {
            let row: Vec<T> = line.chars().map(parse).collect();
            width = row.len();
            height += 1;
            grid.push(row);
        }

        Grid {
            grid,
            width,
            height,
        }
    }

    pub fn get(&self, point: &Point<usize>) -> Option<&T> {
        if let Some(row) = self.grid.get(point.y) {
            row.get(point.x)
        } else {
            None
        }
    }

    pub fn set(&mut self, point: Point<usize>, value: T) {
        if let Some(row) = self.grid.get_mut(point.y) {
            if let Some(cell) = row.get_mut(point.x) {
                *cell = value;
            }
        }
    }

    pub fn neighbours_of_point(
        &self,
        point: &Point<usize>,
        include_diagonals: bool,
    ) -> Vec<Point<usize>> {
        let mut neighbours = vec![];
        if point.x > 0 {
            neighbours.push(Point {
                x: point.x - 1,
                y: point.y,
            });
            if include_diagonals && point.y > 0 {
                neighbours.push(Point {
                    x: point.x - 1,
                    y: point.y - 1,
                });
            }
            if include_diagonals && point.y < self.height - 1 {
                neighbours.push(Point {
                    x: point.x - 1,
                    y: point.y + 1,
                });
            }
        }
        if point.x < self.width - 1 {
            neighbours.push(Point {
                x: point.x + 1,
                y: point.y,
            });
            if include_diagonals && point.y > 0 {
                neighbours.push(Point {
                    x: point.x + 1,
                    y: point.y - 1,
                });
            }
            if include_diagonals && point.y < self.height - 1 {
                neighbours.push(Point {
                    x: point.x + 1,
                    y: point.y + 1,
                });
            }
        }
        if point.y > 0 {
            neighbours.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        if point.y < self.height - 1 {
            neighbours.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }

        neighbours
    }
}
