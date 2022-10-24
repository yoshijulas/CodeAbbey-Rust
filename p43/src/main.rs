fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Dice Rolling

fn main() {
    let num = read_input().trim().parse::<i32>().unwrap();

    for _ in 0..num {
        read_input()
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .map(|x| (x * 6.0).floor() + 1.0)
            .for_each(|x| print!("{x} "));
    }
}
