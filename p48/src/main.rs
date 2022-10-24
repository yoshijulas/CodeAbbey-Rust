fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Collatz Sequence

fn main() {
    let num = read_input().trim().parse::<usize>().unwrap();
    let input_vec: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for i in input_vec.iter().take(num) {
        times_to_get_1(*i);
    }
}

fn times_to_get_1(mut x: i32) {
    let mut count_times = 0;
    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        count_times += 1;
    }
    print!("{count_times} ");
}
