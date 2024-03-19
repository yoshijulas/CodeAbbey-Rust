fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Cards Shuffling

fn main() {
    #[rustfmt::skip]
    let mut deck: [&str; 52] = [
        "CA", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "CT", "CJ", "CQ", "CK",
        "DA", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "DT", "DJ", "DQ", "DK",
        "HA", "H2", "H3", "H4", "H5", "H6", "H7", "H8", "H9", "HT", "HJ", "HQ", "HK",
        "SA", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "ST", "SJ", "SQ", "SK",
    ];

    let input_vec: Vec<i32> = read_input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|num| match num {
            52.. => num % 52,
            _ => num,
        })
        .collect();

    for (i, &position) in input_vec.iter().enumerate().take(52) {
        deck.swap(i, position.try_into().unwrap());
    }

    for card in deck {
        print!("{card} ");
    }
}
