fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Linear Congruential Generator

fn main() {
    let num = read_input().trim().parse::<i32>().expect("Not a number");
    let mut res: Vec<i32> = Vec::new();

    for _ in 0..num {
        let v_inp: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (a, c, m, x_cero, n) = (v_inp[0], v_inp[1], v_inp[2], v_inp[3], v_inp[4]);

        // let mut x_next;
        let mut x_cur = x_cero;
        for _ in 0..n {
            // x_next = (a * x_cur + c) % m;
            // x_cur = x_next;

            x_cur = (a * x_cur + c) % m;
        }

        res.push(x_cur);
    }

    for i in res {
        print!("{} ", i);
    }
}
