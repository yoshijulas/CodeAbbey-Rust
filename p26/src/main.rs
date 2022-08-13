fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

const fn gdc(mut a: i32, mut b: i32) -> i32 {
    let mut r: i32;
    while b != 0 {
        r = b;
        b = a % b;
        a = r;
    }

    a
}

// Greatest Common Divisor

fn main() {
    let num = read_input().trim().parse::<i32>().unwrap();

    let mut res: Vec<(i32, i32)> = Vec::new();

    for _ in 0..num {
        let v: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (a, b) = (v[0], v[1]);

        let gdc = gdc(a, b);

        res.push((gdc, a * b / gdc));
    }

    for i in res {
        print!("({} {}) ", i.0, i.1);
    }
}
