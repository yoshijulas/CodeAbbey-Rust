fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Rotate String

fn main() {
    let number = read_input().trim().parse::<i32>().unwrap();
    let mut res: Vec<String> = Vec::new();

    for _ in 0..number {
        let input = read_input();
        let vec_input: Vec<_> = input.split_whitespace().collect();

        let number: i32 = vec_input[0].trim().parse().unwrap();
        let mut text_input: Vec<char> = vec_input[1].chars().collect();

        let number_usize: usize = number.abs().try_into().unwrap();
        if number >= 0 {
            text_input.rotate_left(number_usize);
        } else {
            text_input.rotate_right(number_usize);
        }

        res.push(text_input.into_iter().collect::<String>());
    }

    for i in res {
        println!("{i}");
    }
}
