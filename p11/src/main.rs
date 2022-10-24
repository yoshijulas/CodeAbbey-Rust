fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Sum of digits

fn main() {
    let number = read_input().trim().parse::<usize>().unwrap();

    let mut res: Vec<i32> = Vec::new();

    for _ in 0..number {
        let vector_input: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (a, b, c) = (vector_input[0], vector_input[1], vector_input[2]);

        let mut op = a * b + c;
        let mut remainder;
        //restart sum value from 0
        let mut sum = 0;
        while op != 0 {
            remainder = op % 10;
            op /= 10;

            //sum the remainder
            sum += remainder;
        }
        res.push(sum);
    }

    for i in res {
        print!("{i} ");
    }
}
