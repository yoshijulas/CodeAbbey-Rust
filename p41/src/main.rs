use std::mem::swap;

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Median of three

fn main() {
    let num = read_input().trim().parse::<i32>().unwrap();

    for _ in 0..num {
        let mut input_vec: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let median = median_of_three(&mut input_vec);

        print!("{median} ");
    }
}

fn _median_of_three_sort(input_vec: &mut [i32]) -> i32 {
    input_vec.sort_unstable();
    input_vec[1]
}

fn median_of_three(input_vec: &mut [i32]) -> i32 {
    let (mut a, mut b, mut c) = (input_vec[0], input_vec[1], input_vec[2]);

    if a > b {
        swap(&mut a, &mut b);
    }
    if b > c {
        swap(&mut b, &mut c);
    }
    if a > b {
        swap(&mut a, &mut b);
    }

    b
}
