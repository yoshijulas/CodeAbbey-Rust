fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Josephus Problem

fn main() {
    let input: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (n, k) = (input[0], (input[1] - 1));

    let mut numbers: Vec<i32> = (1..=n).collect();

    let mut pos = 0;
    let mut counter = 0;
    while numbers.len() != 1 {
        if pos == numbers.len() {
            // reset position if passed over all numbers.len()
            pos = 0;
        }

        if counter == k {
            // if counter == k, then we remove the number in that position
            numbers.remove(pos);
            counter = 0;
        } else {
            counter += 1;
            pos += 1;
        }
    }
    print!("{}", numbers[0]);
}
