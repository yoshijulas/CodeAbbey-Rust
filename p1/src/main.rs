fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let line = read_input();

    let mut answer = 0;

    for word in line.split_whitespace() {
        answer += word.parse::<i32>().expect("Failed to parse word");
    }

    println!("{}", answer);
}