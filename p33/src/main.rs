fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Parity Control

fn main() {
    let input: Vec<u8> = read_input()
        .split_whitespace()
        .map(|x| x.parse::<u8>().expect("Failed to parse input"))
        .collect();

    let binary: Vec<String> = input
        .iter()
        .filter(|x| x.count_ones() % 2 == 0)// remove invalid characters
        .map(|x| format!("{:08b}", x)) // add leading 0's to the binary string
        .collect();

    for i in &binary {
        let a = if i.starts_with('1') {
            i.replacen('1', "0", 1) // remplace leading 1 with 0 to the binary string
        } else {
            i.to_string()
        };
        print!("{}", u8::from_str_radix(&a, 2).unwrap() as char); // convert string to u8 and print ASCII representation
    }
}
