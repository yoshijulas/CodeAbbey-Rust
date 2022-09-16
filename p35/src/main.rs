fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Savings Calculator

fn main() {
    let num: i32 = read_input().trim().parse().unwrap();

    let mut result: Vec<i32> = Vec::new();

    for _ in 0..num {
        let input_vec: Vec<f32> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let (mut start_amount, requiered_amount, interest_rate) =
            (input_vec[0], input_vec[1], input_vec[2]);

        let mut years = 0;
        let porcentage_rate = interest_rate / 100.0;

        while start_amount < requiered_amount {
            start_amount += porcentage_rate * start_amount;
            start_amount = (start_amount * 100.0).floor() / 100.0;

            years += 1;
        }

        result.push(years);
    }

    for i in result {
        print!("{} ", i);
    }
}
