fn read_input() -> String {
    let mut input_str = String::new();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");
    input_str
}

// Neumann's Random Generator

fn main() {
    let _num: i32 = read_input().trim().parse().unwrap(); //Skips numbers of values, first line
    let mut res: Vec<i32> = Vec::new(); //Vector to store the result

    let v: Vec<String> = read_input()
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect(); //Splits the input into a vector of strings

    for i in v {
        //Iterates through the vector
        let mut random_number: i32 = i.parse().unwrap(); //Parses the string into an integer
        let number_string: String = i; //Stores the string in a variable
        let mut counter = 1; //Starts the counter at 1
        let mut seen: Vec<String> = vec![number_string]; //Creates a vector to store the seen numbers
                                                         // seen.push(number_string); //Pushes the number read (as String) into the vector

        loop {
            //Loops until the number is found duppled
            random_number *= random_number; //Squares the number
            let mut rand_num = random_number.to_string(); //Converts the number to a string

            if rand_num.len() < 8 {
                //If the number is less than 8 digits, it adds 0s to the front
                let mut temp: String = String::new();
                for _ in 0..(8 - rand_num.len()) {
                    temp.push('0'); //Stores 0s in a temporary string
                }
                rand_num = temp + &rand_num; //Adds the 0s to the front of the number
            }

            rand_num = rand_num[2..6].to_string(); //Removes the first two bytes and last 2 bytes

            if seen.contains(&rand_num) {
                // If the number is already in the vector, breaks
                break;
            } //else, adds the number to the seen vector

            seen.push(rand_num.clone());

            random_number = rand_num.parse().unwrap(); //Parses the string into an integer

            counter += 1; //Increases the counter
        }

        res.push(counter); //Pushes the counter into the result vector
    }

    for i in res {
        //Prints the result
        print!("{} ", i);
    }
}
