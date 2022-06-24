fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

fn main() {
    let n = read_input().trim().parse::<usize>().unwrap();

    let mut res: Vec<(i32, i32, i32, i32)> = Vec::new();

    for _ in 0..n {
        let v: Vec<i32> = read_input()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (day1, hour1, min1, mut sec1, day2, hour2, min2, mut sec2) =
            (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]);

        sec1 += (day1 * 86400) + (hour1 * 3600) + (min1 * 60);
        sec2 += (day2 * 86400) + (hour2 * 3600) + (min2 * 60);

        //calculate the difference in seconds
        let mut dif = sec2 - sec1;

        //convert to days, hours, minutes and seconds
        let a = dif / 86400;
        dif %= 86400;
        let b = dif / 3600;
        dif %= 3600;
        let c = dif / 60;
        dif %= 60;
        let d = dif;

        res.push((a, b, c, d));
    }

    for i in res {
        print!("({} {} {} {}) ", i.0, i.1, i.2, i.3);
    }
}
