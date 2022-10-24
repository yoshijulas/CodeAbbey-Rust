fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Vowel Count

fn main() {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U', 'y', 'Y'];
    let num: i32 = read_input().trim().parse().unwrap();

    let mut res: Vec<i32> = Vec::new();

    for _ in 0..num {
        let mut vowel_count: i32 = 0;
        let input = read_input();
        for c in input.chars() {
            if vowels.contains(&c) {
                vowel_count += 1;
            }
        }
        res.push(vowel_count);
    }

    for r in res {
        print!("{r} ");
    }
}
