fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

#[derive(Clone, Copy)]
enum Mark {
    X,
    O,
    Empty,
}

// Tic-Tac-Toe

fn main() {
    let num = read_input().trim().parse().unwrap();

    let mut answer: Vec<usize> = Vec::new();

    for _ in 0..num {
        let vec_input: Vec<usize> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut poss: [Mark; 9] = [Mark::Empty; 9];
        let mut won = false;
        let mut number = 0;

        loop {
            if number == 9 {
                answer.push(0);
                break;
            }
            poss[vec_input[number] - 1] = if number % 2 == 0 { Mark::X } else { Mark::O };

            for j in (0..=6).step_by(3) {
                match (poss[j], poss[1 + j], poss[2 + j]) {
                    (Mark::X, Mark::X, Mark::X) | (Mark::O, Mark::O, Mark::O) => {
                        won = true;
                        break;
                    }
                    (_, _, _) => (),
                }
            }
            for j in 0..3 {
                match (poss[j], poss[3 + j], poss[6 + j]) {
                    (Mark::X, Mark::X, Mark::X) | (Mark::O, Mark::O, Mark::O) => {
                        won = true;
                        break;
                    }
                    (_, _, _) => (),
                }
            }

            match (poss[0], poss[4], poss[8]) {
                (Mark::X, Mark::X, Mark::X) | (Mark::O, Mark::O, Mark::O) => won = true,
                (_, _, _) => (),
            }

            match (poss[2], poss[4], poss[6]) {
                (Mark::X, Mark::X, Mark::X) | (Mark::O, Mark::O, Mark::O) => won = true,
                (_, _, _) => (),
            }

            if won {
                break;
            }
            number += 1;
        }

        if number < 9 {
            answer.push(number + 1);
        }
    }

    for res in answer {
        print!("{res} ");
    }
}
