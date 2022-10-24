use std::cmp::min;

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Minimum of Two

fn main() {
    let n: i32 = read_input().trim().parse().unwrap();

    let mut res = Vec::new();

    for _ in 0..n {
        let v: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (x, y) = (v[0], v[1]);

        res.push(min(x, y));
    }

    //print result
    for i in res {
        print!("{i} ");
    }
}
