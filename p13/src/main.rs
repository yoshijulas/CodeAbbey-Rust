fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Weighted sum of digits

fn main() {
    let mut res: Vec<u32> = Vec::new();

    let input = read_input();
    let v = input.split_whitespace();

    for i in v {
        let mut sum = 0;
        for (iter, j) in i.chars().enumerate() {
            if let Some(x) = j.to_digit(10) {
                sum += x * (iter as u32 + 1);
            }
        }
        res.push(sum);
    }

    for i in res {
        print!("{i} ");
    }
}
