#[derive(Debug)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
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

    pub fn get(&self, point: &Point) -> Option<&T> {
        if let Some(row) = self.grid.get(point.y) {
            row.get(point.x)
        } else {
            None
        }
    }

    pub fn neighbours_of_point(&self, point: &Point, include_diagonals: bool) -> Vec<Point> {
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
