fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn is_brackets_match(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in input.chars() {
        match c {
            '(' | '{' | '[' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return false;
                }
            }

            _ => continue,
        }
    }
    stack.is_empty()
}

// Matching Brackets

fn main() {
    let num = read_input().trim().parse().unwrap();
    let mut answer = Vec::new();

    for _ in 0..num {
        let input = read_input();
        answer.push(if is_brackets_match(&input) { 1 } else { 0 });
    }

    for a in answer {
        print!("{} ", a);
    }
}