fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Double Dice Roll

fn main() {
    let num = read_input().trim().parse().unwrap();

    for _ in 0..num {
        let dice: i32 = read_input()
            .split_whitespace()
            .map(|inp| inp.parse().unwrap())
            .map(|num: i32| (num % 6) + 1)
            .sum();

        print!("{dice} ");
    }
}
