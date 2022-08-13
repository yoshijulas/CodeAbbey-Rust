use num_bigint::BigInt;

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Modular Calculator

fn main() {
    let mut res: BigInt = read_input().trim().parse().unwrap();

    loop {
        let input = read_input();
        let v: Vec<_> = input.split_whitespace().collect();

        let op: char = v[0].chars().next().unwrap();
        let num: i32 = v[1].parse().unwrap();

        match op {
            '+' => res += num,
            '-' => res -= num,
            '*' => res *= num,
            '/' => res /= num,
            '%' => {
                res %= num;
                break;
            }
            _ => continue,
        }
    }

    print!("{}", res);
}