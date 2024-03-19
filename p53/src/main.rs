fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// King and Queen

fn main() {
    let num: usize = read_input().trim().parse().unwrap();
    let mut result: Vec<bool> = Vec::new();

    for _ in 0..num {
        let line: Vec<String> = read_input()
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect();
        result.push(can_queen_capture_king(&line[0], &line[1]));
    }

    for res in result {
        if res {
            print!("Y ");
        } else {
            print!("N ");
        }
    }
}

fn can_queen_capture_king(king: &str, queen: &str) -> bool {
    let (king_letter, king_digit) = (
        (king.chars().next().unwrap()) as isize,
        king.chars().nth(1).unwrap() as isize,
    );

    let (queen_letter, queen_digit) = (
        (queen.chars().next().unwrap()) as isize,
        queen.chars().nth(1).unwrap() as isize,
    );

    (king_letter == queen_letter)
        || (king_digit == queen_digit)
        || (king_letter - queen_letter).abs() == (king_digit - queen_digit).abs()
}
