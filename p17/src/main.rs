fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Array Checksum

fn main() {
    let num: i32 = read_input().trim().parse().unwrap();
    let mut checksum = 0;
    let seed = 113;
    let modulo = 10_000_007;

    let v: Vec<i64> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for i in 0..num {
        checksum = ((checksum + v[i as usize]) * seed) % modulo;
    }

    print!("{checksum}");
}
