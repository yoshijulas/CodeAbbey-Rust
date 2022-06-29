fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn calc_root(x: f64, num: u32) -> f64 {
    let mut r = 1.0;
    for _ in 0..num {
        r = (r + x / r) / 2.0;
    }
    r
}

fn main() {
    let num: i32 = read_input().trim().parse::<i32>().unwrap();
    let mut result: Vec<f64> = Vec::new();

    for _ in 0..num {
        let input_vec: Vec<f64> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        result.push(calc_root(input_vec[0], input_vec[1] as u32));
    }

    for r in result {
        print!("{} ", r);
    }
}