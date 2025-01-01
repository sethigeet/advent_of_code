use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use aoc2024::grid::{Grid, Point};
use itertools::Itertools;

fn get_regions(input: &String) -> HashMap<char, Vec<Vec<Point>>> {
    let grid: Grid<char> = Grid::from_str(input, |c| c);

    let mut regions: HashMap<char, Vec<Vec<Point>>> = HashMap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    for i in 0..grid.height {
        for j in 0..grid.width {
            let point = Point::new(j, i);
            if visited.contains(&point) {
                continue;
            }

            let mut region: Vec<Point> = vec![];

            // BFS
            let mut queue: VecDeque<Point> = VecDeque::from_iter([point]);
            let mut inner_visited: HashSet<Point> = HashSet::new();
            while let Some(point) = queue.pop_front() {
                if inner_visited.contains(&point) {
                    continue;
                }

                inner_visited.insert(point);
                let current_val = grid.get(&point).unwrap();
                for neighbour in grid.neighbours_of_point(&point, false) {
                    let neighbour_val = grid.get(&neighbour).unwrap();
                    if *current_val == *neighbour_val && !inner_visited.contains(&neighbour) {
                        queue.push_back(neighbour);
                    }
                }
                region.push(point);
            }

            visited = visited.union(&inner_visited).cloned().collect();
            if let Some(v) = regions.get_mut(&grid.grid[i][j]) {
                v.push(region);
            } else {
                regions.insert(grid.grid[i][j], vec![region]);
            }
        }
    }

    regions
}

fn calculate_region_perimeter(region: &Vec<Point>) -> i64 {
    let mut perimeter = 0;
    for loc in region {
        if !region.contains(&Point::new(loc.x + 1, loc.y)) {
            perimeter += 1;
        }
        if !region.contains(&Point::new((loc.x as i64 - 1) as usize, loc.y)) {
            perimeter += 1;
        }
        if !region.contains(&Point::new(loc.x, loc.y + 1)) {
            perimeter += 1;
        }
        if !region.contains(&Point::new(loc.x, (loc.y as i64 - 1) as usize)) {
            perimeter += 1;
        }
    }

    perimeter
}

fn part1(input: &String) -> i64 {
    let regions = get_regions(input);

    let mut total_cost = 0;
    for (_, regions) in regions {
        for region in regions {
            let area = region.len() as i64;
            let perimiter = calculate_region_perimeter(&region);
            total_cost += area * perimiter;
        }
    }

    total_cost
}

fn calculate_region_sides(region: &Vec<Point>) -> i64 {
    let left_sides = region
        .iter()
        .filter(|p| !region.contains(&Point::new((p.x as i64 - 1) as usize, p.y)))
        .sorted_by(|a, b| {
            if a.x == b.x {
                a.y.cmp(&b.y)
            } else {
                a.x.cmp(&b.x)
            }
        })
        .tuple_windows()
        .map(|(a, b)| if a.x == b.x && b.y - a.y == 1 { 0 } else { 1 })
        .sum::<i64>()
        + 1;

    let right_sides = region
        .iter()
        .filter(|p| !region.contains(&Point::new((p.x as i64 + 1) as usize, p.y)))
        .sorted_by(|a, b| {
            if a.x == b.x {
                a.y.cmp(&b.y)
            } else {
                a.x.cmp(&b.x)
            }
        })
        .tuple_windows()
        .map(|(a, b)| if a.x == b.x && b.y - a.y == 1 { 0 } else { 1 })
        .sum::<i64>()
        + 1;

    let top_sides = region
        .iter()
        .filter(|p| !region.contains(&Point::new(p.x, (p.y as i64 - 1) as usize)))
        .sorted_by(|a, b| {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        })
        .tuple_windows()
        .map(|(a, b)| if a.y == b.y && b.x - a.x == 1 { 0 } else { 1 })
        .sum::<i64>()
        + 1;

    let bottom_sides = region
        .iter()
        .filter(|p| !region.contains(&Point::new(p.x, (p.y as i64 + 1) as usize)))
        .sorted_by(|a, b| {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        })
        .tuple_windows()
        .map(|(a, b)| if a.y == b.y && b.x - a.x == 1 { 0 } else { 1 })
        .sum::<i64>()
        + 1;

    left_sides + right_sides + top_sides + bottom_sides
}

fn part2(input: &String) -> i64 {
    let regions = get_regions(input);

    let mut total_cost = 0;
    for (_, regions) in regions {
        for region in regions {
            let area = region.len() as i64;
            let perimiter = calculate_region_sides(&region);
            total_cost += area * perimiter;
        }
    }

    total_cost
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
