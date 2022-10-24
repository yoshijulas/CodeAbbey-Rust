fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Triangles

fn main() {
    let number: i32 = read_input().trim().parse().unwrap();

    let mut res: Vec<i32> = Vec::new();

    for _ in 0..number {
        let vector_input: Vec<f32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();

        let (a, b, c) = (vector_input[0], vector_input[1], vector_input[2]);

        if a + b > c && a + c > b && b + c > a {
            res.push(1);
        } else {
            res.push(0);
        }
    }

    //print result
    for i in res {
        print!("{i} ");
    }
}
