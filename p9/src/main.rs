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

        let (a, b, c) = (v[0], v[1], v[2]);

        if a + b > c && a + c > b && b + c > a {
            res.push(1);
        } else {
            res.push(0);
        }
    }

    //print result
    for i in res {
        print!("{} ", i);
    }
}
