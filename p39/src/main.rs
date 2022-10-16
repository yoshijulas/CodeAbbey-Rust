fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Share Price Volatility

fn main() {
    // (2500 + 250 + 250) / 3 = 1000 Average
    // (1000 - 2500)^2 = 2250000 firt, second, third
    // (2250000 + 562500 + 562500) / 3 = 1125000 Average of that
    // sqrt(mean({D})) = sqrt(1125000) = 1061 square root of that

    let mut answer: Vec<(String, i32)> = Vec::new();
    let input = read_input().trim().parse::<i32>().unwrap();

    for _ in 0..input {
        let mut input_vec: Vec<String> = read_input()
            .split_whitespace()
            .map(|s| s.parse::<String>().unwrap())
            .rev()
            .collect();

        let name = input_vec.pop().unwrap();
        let d = standard_deviation(&input_vec);

        if d >= 4.0 {
            answer.push((name, d as i32));
        }
    }

    for i in answer {
        print!("{} ", i.0);
    }

    // println!("{}", answer.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0);
}

fn standard_deviation(vector: &[String]) -> f64 {
    let values: Vec<f64> = vector.iter().map(|x| x.parse::<f64>().unwrap()).collect();

    let mean = get_mean(&values);

    let quadratic_dist: f64 =
        values.iter().map(|x| (mean - x).powi(2)).sum::<f64>() / values.len() as f64;

    let deviation = quadratic_dist.sqrt();

    deviation / (mean * 0.01)
}

fn get_mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}
