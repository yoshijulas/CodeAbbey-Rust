fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Maximum of array

fn main() {
    let input_vec: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    print!("{} {}", input_vec.iter().max().unwrap(), input_vec.iter().min().unwrap());
}