fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Sum in Loop

fn main() {
    let _number = read_input().trim().parse::<i32>().unwrap();

    let input: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // for _ in 0..number {
    //     let number = input.next().unwrap().parse::<i32>().unwrap();
    //     sum += number;
    // }

    let sum: i32 = input.iter().sum();

    println!("{sum}");
}
