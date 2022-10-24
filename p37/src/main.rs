fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Mortgage Calculator

fn main() {
    let input_vec: Vec<u32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (purchase_price, interest_rate, months_to_pay) =
        (input_vec[0] as f32, input_vec[1] as f32, input_vec[2]);

    // binary_search(purchase_price, months_to_pay, interest_rate);

    brute_force(purchase_price, months_to_pay, interest_rate);
}

fn brute_force(purchase_price: f32, months_to_pay: u32, interest_rate: f32) {
    for i in 0..100_000_000 {
        let mut new_price: f32 = purchase_price;
        for _ in 0..months_to_pay {
            new_price += new_price * (interest_rate / 12.0) / 100.0;
            new_price -= i as f32;
            new_price = new_price.round();
        }

        if new_price <= 0.0 {
            println!("{i}");
            break;
        }
    }
}

fn _binary_search(purchase_price: f32, months_to_pay: u32, interest_rate: f32) {
    // use Binary search to find the monthly payment
    let mut min: f32 = 0.0;
    let mut max: f32 = 100_000_000.0;
    loop {
        let middle: f32 = (min + max) / 2.0;
        let middle_rounded: i32 = middle.round() as i32;
        let mut new_price = purchase_price;

        for _ in 0..months_to_pay {
            new_price += new_price * (interest_rate / 12.0) / 100.0;
            new_price -= middle_rounded as f32;
            if new_price < 0.0 {
                break;
            }
            new_price = new_price.round();
        }

        if (new_price).abs() <= 16.0 {
            println!("{middle_rounded}");
            break;
        }

        if new_price > 0.0 {
            min = middle_rounded as f32;
        } else {
            max = middle_rounded as f32;
        }
    }
}
