fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let num: i32 = read_input().trim().parse().unwrap();

    let mut result: Vec<i32> = Vec::new();

    for _ in 0..num {
        let v: Vec<i64> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let sum: i64 = v.iter().sum();
        let avg: f64 = sum as f64 / (v.len() as f64 - 1.0);

        result.push(avg.round() as i32);
    }

    for res in result {
        print!("{} ", res);
    }
}
