fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

struct Posibilities {
    numbers: u32,
    true_values: u32,
}

// Code Guesser

fn main() {
    let mut num_a = [true; 10];
    let mut num_b = [true; 10];
    let mut num_c = [true; 10];
    let mut num_d = [true; 10];
    let p = read_input().trim().parse::<u32>().unwrap();

    let mut possible_combinations: Vec<Posibilities> = Vec::new();

    // remove the ones that are not possible
    for _ in 0..p {
        let input_vec: Vec<u32> = read_input()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (numbers, true_values) = (input_vec[0], input_vec[1]);
        assert!(
            numbers.to_string().len() == 4,
            "Should be only 4 digits numbers"
        );

        let combination_tuple = Posibilities {
            numbers,
            true_values,
        };

        if combination_tuple.true_values == 0 {
            let numbers = &combination_tuple.numbers.to_string();

            for (i, c) in numbers.chars().enumerate() {
                match i {
                    0 => num_a[c.to_digit(10).unwrap() as usize] = false,
                    1 => num_b[c.to_digit(10).unwrap() as usize] = false,
                    2 => num_c[c.to_digit(10).unwrap() as usize] = false,
                    3 => num_d[c.to_digit(10).unwrap() as usize] = false,
                    _ => unreachable!("The number should only be 4 digits"),
                }
            }
        }
        possible_combinations.push(combination_tuple);
    }

    // Logic

    logic_to_solve(&num_a, &num_b, &num_c, &num_d, &possible_combinations);
}

fn logic_to_solve(
    num_a: &[bool; 10],
    num_b: &[bool; 10],
    num_c: &[bool; 10],
    num_d: &[bool; 10],
    possible_combinations: &[Posibilities],
) {
    for (a, i) in num_a.iter().enumerate() {
        for (b, j) in num_b.iter().enumerate() {
            for (c, k) in num_c.iter().enumerate() {
                for (d, l) in num_d.iter().enumerate() {
                    if *i && *j && *k && *l {
                        let bforce_all_string = format!("{a}{b}{c}{d}");

                        let mut valid = true;

                        for i in possible_combinations {
                            let (input_number_tries, number_of_true_values) =
                                (i.numbers, i.true_values);

                            if input_number_tries
                                .to_string()
                                .chars()
                                .zip(bforce_all_string.chars())
                                .filter(|(a, b)| a == b)
                                .count() // counts the numbers of true values
                                != number_of_true_values as usize
                            {
                                valid = false;
                                break;
                            }
                        }

                        if valid {
                            println!("{bforce_all_string}");
                        }
                    }
                }
            }
        }
    }
}
