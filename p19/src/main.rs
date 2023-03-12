fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn is_brackets_match(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let brackets = input.chars().all(|c| match c {
        '(' | '{' | '[' | '<' => {
            stack.push(c);
            true
        }
        ')' => stack.pop() == Some('('),
        '}' => stack.pop() == Some('{'),
        ']' => stack.pop() == Some('['),
        '>' => stack.pop() == Some('<'),
        _ => true,
    });
    stack.is_empty() && brackets
}

// Matching Brackets

fn main() {
    let num = read_input().trim().parse().unwrap();
    let mut answer = Vec::new();

    for _ in 0..num {
        let input = read_input();
        answer.push(i32::from(is_brackets_match(&input)));
    }

    for a in answer {
        print!("{a} ");
    }
}
