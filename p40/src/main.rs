fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Paths in the Grid / unique paths with obstacles

// @ + + + +
// + + + X X
// + X + + +
// + + + X +
// + X + + X
// + + + + $

fn main() {
    let num: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut vec: Vec<Vec<i32>> = vec![];

    for _ in 0..num[0] {
        vec.push(
            read_input()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .map(|x| match x {
                    '@' => 1,
                    'X' => -1,
                    '$' | '+' => 0,
                    _ => unreachable!("Should not have more types of paths, else, modify the code"),
                })
                .collect(),
        );
    }

    // println!("{vec:?}");

    // This is an example for calling this function:
    // let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 0]];

    println!("{}", unique_paths_with_obstacles(vec));
}

fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = obstacle_grid;
    let m = grid.len();
    let n = grid[0].len();

    for row in 0..m {
        for col in 0..n {
            if grid[row][col] >= 0 {
                if row > 0 && grid[row - 1][col] >= 0 {
                    grid[row][col] += grid[row - 1][col];
                }
                if col > 0 && grid[row][col - 1] >= 0 {
                    grid[row][col] += grid[row][col - 1];
                }
            }
        }
    }

    grid[m - 1][n - 1]
}
