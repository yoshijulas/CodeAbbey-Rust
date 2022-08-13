fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Linear Function

fn main() {
    let n: i32 = read_input().trim().parse().unwrap();

    let mut res: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        let v: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (x1, y1, x2, y2) = (v[0], v[1], v[2], v[3]);

        //slope
        let m = (y2 - y1) / (x2 - x1);
        //intersection
        let b = y1 - m * x1;

        res.push((m, b));
    }

    for i in res {
        print!("({} {}) ", i.0, i.1);
    }
}