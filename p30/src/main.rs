fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Reverse String

fn main() {
    let input: String = read_input();
    let rev_input: String = input.chars().rev().collect();
    print!("{rev_input}");
}
