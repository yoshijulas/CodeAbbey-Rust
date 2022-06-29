fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let number = read_input().trim().parse::<i32>().unwrap();
    let mut sum = 0;

    let input = read_input();
    let mut input = input.split_whitespace();

    for _ in 0..number {
        let number = input.next().unwrap().parse::<i32>().unwrap();
        sum += number;
    }

    println!("{}", sum);
}