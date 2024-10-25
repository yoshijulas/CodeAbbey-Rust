#![allow(clippy::cast_possible_truncation)]

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Mortgage Calculator

fn mortage(loan_size: f32, interest_rate: f32, n_periods: f32) -> i32 {
    let rate = interest_rate / 1200.0; // 12 months / 100 for percentage
    let amortization =
        loan_size * ((rate * (1.0 + rate).powf(n_periods)) / ((1.0 + rate).powf(n_periods) - 1.0));

    amortization.ceil() as i32
}

fn main() {
    let inp_vec: Vec<f32> = read_input()
        .split_whitespace()
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    let (loan_size, interest_rate, lenght_in_months) = (inp_vec[0], inp_vec[1], inp_vec[2]);

    print!("{} ", mortage(loan_size, interest_rate, lenght_in_months));
}
