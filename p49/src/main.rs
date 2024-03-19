fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Rock Paper Scissors

fn main() {
    let num = read_input().trim().parse().unwrap();
    let mut output_vec: Vec<i32> = Vec::new();

    for _ in 0..num {
        let input = read_input();
        let input_vec: Vec<&str> = input.split_whitespace().collect();

        let mut a = 0;
        let mut b = 0;
        for i in input_vec {
            let a_inp = i.chars().next().unwrap();
            let b_inp = i.chars().nth(1).unwrap();

            // println!("{i}, {a_inp} {b_inp}");

            match (a_inp, b_inp) {
                ('R', 'S') | ('S', 'P') | ('P', 'R') => {
                    if b >= 1 {
                        b -= 1;
                    } else {
                        a += 1;
                    }
                }
                ('S', 'R') | ('P', 'S') | ('R', 'P') => {
                    if a >= 1 {
                        a -= 1;
                    } else {
                        b += 1;
                    }
                }
                (_, _) => {}
            }
        }
        if a > b {
            output_vec.push(1);
        } else {
            output_vec.push(2);
        }
    }

    for i in output_vec {
        print!("{i} ");
    }
}
