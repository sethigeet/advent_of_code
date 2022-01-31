use anyhow::Result;

fn get_num_trees(forest: &[Vec<char>], slope: (usize, usize)) -> u32 {
    let forest_width = forest[0].len();

    let mut pos: (usize, usize) = (0, 0);
    let mut trees = 0;
    while pos.1 < forest.len() {
        // make sure the pos does not go out of the forest
        if pos.0 > (forest_width - 1) {
            pos.0 = (pos.0 % (forest_width - 1)) - 1;
        }

        let curr = &forest[pos.1][pos.0];
        if *curr == '#' {
            trees += 1;
        }

        // move to new pos
        pos.0 += slope.0;
        pos.1 += slope.1;
    }

    trees
}

fn main() -> Result<()> {
    let forest: Vec<Vec<char>> = std::fs::read_to_string("./data/inputs/3.txt")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    println!("Part 1 -> {}", get_num_trees(&forest, slopes[1]));

    println!(
        "Part 2 -> {}",
        slopes
            .into_iter()
            .fold(1, |acc, slope| acc * get_num_trees(&forest, slope))
    );

    Ok(())
}
