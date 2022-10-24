fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Modulo and time difference

fn main() {
    let number = read_input().trim().parse::<usize>().unwrap();

    let mut res: Vec<(i32, i32, i32, i32)> = Vec::new();

    for _ in 0..number {
        let vector_input: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (day1, hour1, min1, mut sec1, day2, hour2, min2, mut sec2) = (
            vector_input[0],
            vector_input[1],
            vector_input[2],
            vector_input[3],
            vector_input[4],
            vector_input[5],
            vector_input[6],
            vector_input[7],
        );

        sec1 += (day1 * 86400) + (hour1 * 3600) + (min1 * 60);
        sec2 += (day2 * 86400) + (hour2 * 3600) + (min2 * 60);

        //calculate the difference in seconds
        let mut dif = sec2 - sec1;

        //convert to days, hours, minutes and seconds
        let days_number = dif / 86400;
        dif %= 86400;
        let hours_number = dif / 3600;
        dif %= 3600;
        let minutes_number = dif / 60;
        dif %= 60;
        let seconds_number = dif;

        res.push((days_number, hours_number, minutes_number, seconds_number));
    }

    for i in res {
        print!("({} {} {} {}) ", i.0, i.1, i.2, i.3);
    }
}
