fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Body Mass Index

fn main() {
    let num = read_input().trim().parse::<i32>().unwrap();
    let mut res: Vec<&str> = Vec::new();

    for _ in 0..num {
        let v: Vec<f32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();

        let (weight, height) = (v[0], v[1]);

        let bmi = weight / (height * height);

        if bmi < 18.5 {
            res.push("under");
        } else if (18.5..25.0).contains(&bmi) {
            res.push("normal");
        } else if (25.0..30.0).contains(&bmi) {
            res.push("over");
        } else {
            res.push("obese");
        }
    }

    for i in res {
        print!("{i} ");
    }
}
