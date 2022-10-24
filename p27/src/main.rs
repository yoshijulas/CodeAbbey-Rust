fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Bubble Sort

fn main() {
    let _num = read_input();

    let input_vec: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (_sorted_vec, passes, counter, _max_counter) = bubble_sort(input_vec);

    println!("{passes} {counter}");
    // println!("{:?}", _sorted_vec);
    // println!("{}", max_counter);
}

fn bubble_sort(mut input_vec: Vec<i32>) -> (Vec<i32>, i32, i32, i32) {
    let mut counter = 0;

    let mut changed = true;
    let mut passes = 0;
    let mut max_counter = 0;

    let mut n = input_vec.len() - 1;

    while changed {
        changed = false;

        for i in 0..n {
            if input_vec[i] > input_vec[i + 1] {
                input_vec.swap(i, i + 1);
                counter += 1;
                changed = true;
            }
            max_counter += 1;
        }
        n -= 1; // one less element to check, improves performance, numeric triangle () every pass removed
                // 374 -> 238 for 23 values
        passes += 1;
    }
    (input_vec, passes, counter, max_counter)
}
