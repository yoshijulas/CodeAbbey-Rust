fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn bubble_sort(mut input_vec: Vec<i32>) -> Vec<i32> {
    let mut changed = true;

    let mut n = input_vec.len() - 1;

    while changed {
        changed = false;

        for i in 0..n {
            if input_vec[i] > input_vec[i + 1] {
                input_vec.swap(i, i + 1);
                changed = true;
            }
        }
        n -= 1; // one less element to check, improves performance, numeric triangle () every pass removed
                // 374 -> 238 for 23 values
    }
    input_vec
}

// Sort with Indexes

fn main() {
    let _num = read_input();

    let mut res: Vec<i32> = Vec::new();

    let input_vec: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let sorted_vec = bubble_sort(input_vec.clone());

    for i in sorted_vec {
        for (j, num) in input_vec.iter().enumerate() {
            if i == *num {
                res.push(j as i32 + 1);
            }
        }
    }

    for i in res {
        print!("{i} ");
    }
}
