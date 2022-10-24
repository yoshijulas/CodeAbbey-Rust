fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Quadratic Equation

fn main() {
    let num = read_input().trim().parse::<i32>().unwrap();

    for _ in 0..num {
        let input_vec: Vec<f32> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let (a, b, c) = (input_vec[0], input_vec[1], input_vec[2]);

        let x = solve_equation(a, b, c);

        print!("{} {}; ", x.0, x.1);
    }
}

fn solve_equation(a: f32, b: f32, c: f32) -> (String, String) {
    let x1_string;
    let x2_string;
    let discriminant = b.mul_add(b, -4.0 * a * c);
    let a2 = 2.0 * a;
    if discriminant < 0.0 {
        // complex numbers
        let mut result = -b / a2;
        if result == 0.0 && result.is_sign_negative() {
            result = result.abs();
        }
        x1_string = format!("{result}+{}i", (-discriminant).sqrt() / a2);
        x2_string = format!("{result}-{}i", (-discriminant).sqrt() / a2);
    } else {
        let temp1 = (-b + b.mul_add(b, -4.0 * a * c).sqrt()) / a2;
        let temp2 = (-b - b.mul_add(b, -4.0 * a * c).sqrt()) / a2;
        if temp1 > temp2 {
            x1_string = format!("{temp1}");
            x2_string = format!("{temp2}");
        } else {
            x1_string = format!("{temp2}");
            x2_string = format!("{temp1}");
        }
    }
    (x1_string, x2_string)
}
