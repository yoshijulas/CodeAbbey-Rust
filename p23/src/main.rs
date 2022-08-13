fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn checksum(vec: &[i32]) -> i32 {
    let seed = 113;
    let modulo = 10_000_007;
    vec.iter()
        .fold(0, |prev, curr| ((prev + curr) * seed) % modulo)
}

// Bubble in Array

fn main() {
    let input = read_input();
    let mut input_vec: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    input_vec.pop();

    let n = input_vec.len() - 1;
    let mut counter = 0;

    for i in 0..n {
        if input_vec[i] > input_vec[i + 1] {
            input_vec.swap(i, i + 1);
            counter += 1;
        }
    }

    print!("{} {}", counter, checksum(&input_vec));
}
