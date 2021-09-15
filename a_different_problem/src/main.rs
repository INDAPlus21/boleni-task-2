use std::io;

fn main() {

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("There was something wrong with the input.");

        let numbers = input.trim().split(" ").collect::<Vec<&str>>();
        let n1 = numbers[0].parse::<i64>().expect("Please input a valid number.");
        let n2 = numbers[1].parse::<i64>().expect("Please input a valid number.");

        println!("{}",
        if n1-n2 > 0 { // There are problably built-in features for abs value but eh-
            n1-n2
        } else {
            (n1-n2)*-1
        });
    }
}
