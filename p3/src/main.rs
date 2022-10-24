fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Sums in Loop

fn main() {
    let n: i32 = read_input().trim().parse().unwrap();

    let mut res = Vec::new();

    for _ in 0..n {
        let v: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        res.push(v[0] + v[1]);
    }

    //print result
    for i in res {
        print!("{i} ");
    }
}
