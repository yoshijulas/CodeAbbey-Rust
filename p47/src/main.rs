fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Caesar Shift Cipher

fn main() {
    let mut answers: Vec<String> = Vec::new();

    let num: Vec<u8> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for _ in 0..num[0] {
        let text_input: String = read_input()
            .trim()
            .chars()
            .map(|x| x as u8)
            .map(|x| match x {
                b'.' | b' ' => x,
                _ => {
                    let dif: u8 = x - num[1];
                    if dif < b'A' {
                        (b'Z' + 1) - (num[1] - (x - b'A'))
                    } else {
                        dif
                    }
                }
            })
            .map(|x| x as char)
            .collect();

        answers.push(text_input);
    }

    for answer in answers {
        print!("{answer} ");
    }
}
