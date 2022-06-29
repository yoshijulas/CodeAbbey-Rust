fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let mut res: Vec<i32> = Vec::new();
    let v: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let n: i32 = v[0];

    for i in 1..n {
        let f = v[i as usize];
        let c = (f - 32) as f32 / 1.8;

        res.push(c.round() as i32);
    }

    //print result
    for i in res {
        print!("{} ", i);
    }
}