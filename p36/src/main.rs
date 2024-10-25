fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Code Guesser

fn filter_tries(posibilities: &str, guess: &str) -> usize {
    posibilities
        .chars()
        .zip(guess.chars())
        .filter(|(poss, gues)| poss == gues)
        .count()
}

fn guesser(inp_vec: Vec<(String, usize)>, lenght: usize) -> String {
    let mut posibilities: Vec<String> = (0..(10_i32.pow(lenght.try_into().unwrap())))
        .map(|n| format!("{n:0lenght$}"))
        .collect();

    for (guess, correct_count) in inp_vec {
        posibilities.retain(|num| filter_tries(num, &guess) == correct_count);
    }

    posibilities[0].clone()
}

fn main() {
    let num = read_input().trim().parse().unwrap();

    let mut inp_vec: Vec<(String, usize)> = Vec::new();

    let mut lenght: usize = 0;

    for _ in 0..num {
        let read: Vec<String> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<String>().unwrap())
            .collect();

        // takes the lenght of the first element as the lenght of the rest, works any lenght number
        lenght = read[0].len();

        inp_vec.push((
            format!("{:0lenght$}", read[0]),
            read[1].parse::<usize>().unwrap(),
        ));
    }

    print!("{}", guesser(inp_vec, lenght));
}
