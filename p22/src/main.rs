fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Two Printers

fn main() {
    let num = read_input().trim().parse::<i32>().unwrap();

    let mut res: Vec<i32> = Vec::new();

    for _ in 0..num {
        let input = read_input();
        let input_vec: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (mut count_x, mut count_y, pages) = (input_vec[0], input_vec[1], input_vec[2]);
        let (x, y) = (count_x, count_y);

        for _ in 1..pages {
            if count_x > count_y {
                count_y += y;
            } else {
                count_x += x;
            }
        }

        if count_x > count_y {
            res.push(count_y);
        } else {
            res.push(count_x);
        };
    }

    for i in res {
        print!("{} ", i);
    }
}
