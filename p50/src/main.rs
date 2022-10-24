fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Palindromes

fn main() {
    let num = read_input().trim().parse::<usize>().unwrap();

    for _ in 0..num {
        let input_vec: Vec<char> = read_input()
            .trim()
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect();

        let answer = input_vec
            .clone()
            .into_iter()
            .rev()
            .zip(input_vec)
            .all(|(x, y)| x == y);

        if answer {
            print!("Y ");
        } else {
            print!("N ");
        }
    }
}
