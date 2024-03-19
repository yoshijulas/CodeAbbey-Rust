use std::cmp::Ordering;

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Pythagorean Theorem

fn main() {
    let num = read_input().trim().parse().unwrap();
    let mut output_vec: Vec<char> = Vec::new();

    for _ in 0..num {
        let input_vec: Vec<i32> = read_input()
            .split_whitespace()
            .map(|ch| ch.parse().unwrap())
            .collect();

        let (short, long, hipo) = (input_vec[0], input_vec[1], input_vec[2]);

        let result = match (short.pow(2) + long.pow(2)).cmp(&hipo.pow(2)) {
            Ordering::Equal => 'R',
            Ordering::Greater => 'A',
            Ordering::Less => 'O',
        };
        output_vec.push(result);
    }

    for i in output_vec {
        print!("{i} ");
    }
}
