fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Array Counters

fn main() {
    let num = read_input();
    let num: Vec<i32> = num
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut res: Vec<i32> = vec![0; num[1].try_into().unwrap()];

    let input = read_input();
    let input: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();


    for i in input {
        res[i as usize - 1] += 1;
    }

    for r in res {
        print!("{} ", r);
    }
}
