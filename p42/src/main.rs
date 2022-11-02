fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Blackjack Counting

// input data:
// 4
// A T
// 2 K 4
// 3 A Q 8
// A 3 3 3 A

// answer:
// 21 16 Bust 21

const LIMIT: u32 = 21;

fn main() {
    let n: u32 = read_input().trim().parse().unwrap();
    let mut answer_vec: Vec<String> = Vec::new();

    for _ in 0..n {
        let mut sum = 0_u32;
        let mut ace = 0_u32;
        let input_vec: Vec<char> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for card in input_vec {
            match card {
                '1'..='9' => sum += card.to_digit(10).unwrap(),
                'T' | 'J' | 'Q' | 'K' => sum += 10_u32,
                'A' => ace += 1_u32,
                _ => unreachable!("Not a card"),
            };
        }

        for _ in 0..ace {
            if sum + 11 <= LIMIT {
                sum += 11;
            } else {
                sum += 1;
            }
        }

        if sum > LIMIT {
            answer_vec.push("Bust".to_string());
        } else {
            answer_vec.push(sum.to_string());
        }
    }

    for ans in answer_vec {
        print!("{ans} ");
    }
}
