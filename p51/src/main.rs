use std::usize;

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Dungeons and Dragons Dice

fn main() {
    let mut input_lines: Vec<Vec<usize>> = Vec::new();

    for _ in 0..3 {
        let mut temp_input: Vec<usize> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        temp_input.pop();

        input_lines.push(temp_input);
    }

    for line in input_lines {
        let dice_calculation = calculate_dice(line);
        print!("{}d{} ", dice_calculation.0, dice_calculation.1);
    }
}

fn calculate_dice(line: Vec<usize>) -> (usize, usize) {
    let numeric_average = line.iter().sum::<usize>() as f32 / line.len() as f32;

    // Bruteforce
    // 2, 4, 6, 8, 10, 12 sides
    let dice_sides = [2, 4, 6, 8, 10, 12];
    let mut less_difference = numeric_average;
    let mut best_posibility = (0, 0);

    for sides in dice_sides {
        // 1 - 5 dices
        for dice in 1..=5 {
            // https://anydice.com/articles/dice-and-averages/
            // all dices start at 1
            // sides = max sides in that dice
            // dice = amount of dices
            let avg_numbers = (dice * (1 + sides)) as f32 / 2.0;

            let diff = (avg_numbers - numeric_average).abs();

            if diff < less_difference {
                less_difference = diff;
                best_posibility = (dice, sides);
            }
        }
    }
    best_posibility
}
