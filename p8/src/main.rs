fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let n: i32 = read_input().trim().parse().unwrap();

    let mut res: Vec<i32> = Vec::new();

    for _ in 0..n {
        let v: Vec<f32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();

        let (first_value, step, c) = (v[0], v[1], v[2]);

        let mut sum = first_value + (c - 1.0) * step;
        sum = c * (first_value + sum) / 2.0;

        res.push(sum.round() as i32);
    }

    //print result
    for i in res {
        print!("{} ", i);
    }
}