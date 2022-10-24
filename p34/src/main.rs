#![allow(clippy::many_single_char_names)]

fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn f(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    // a.mul_add(x, b * x.powf(1.5)) - c * (-x / 50.0).exp() - d
    // A * x + B * sqrt(x ^ 3) - C * exp(-x / 50) - D = 0
    c.mul_add(-(-x / 50.0).exp(), a.mul_add(x, b * x.powf(1.5))) - d
}

// Binary Search

fn main() {
    let num = read_input().trim().parse().expect("Failed to parse");

    let mut res_vec: Vec<f64> = Vec::new();

    for _ in 0..num {
        let input_vec: Vec<f64> = read_input()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // Equation for the problem to solve
        // A * x + B * sqrt(x ^ 3) - C * exp(-x / 50) - D = 0;
        let (a, b, c, d) = (input_vec[0], input_vec[1], input_vec[2], input_vec[3]);

        let mut x_min: f64 = 0.0;
        let mut x_max: f64 = 100.0;
        let mut x_middle: f64;
        loop {
            x_middle = (x_min + x_max) / 2.0;

            // Test
            // println!("xmin {}", x_min);
            // println!("xmax {}", x_max);
            // println!("x_mid {}", x_middle);

            let y = f(x_middle, a, b, c, d);
            if y.abs() < 1e-7 {
                break;
            }

            // Test
            // println!("f(x_mid){}", y);

            if y < 0.0 {
                x_min = x_middle;
            } else {
                x_max = x_middle;
            }
        }
        res_vec.push(x_middle);
    }
    for i in res_vec {
        print!("{i:.12} ");
    }
}
