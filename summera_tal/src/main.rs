use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error with input");
    let n: u32 = n.trim().parse().expect("Invalid value n");

    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("Error with input");
    let numbers: Vec<&str> = numbers.trim().split(" ").collect();

    let mut sorted_numbers = Vec::new();

    for num in numbers {
        sorted_numbers.push(num.trim().parse::<u32>().expect("Invalid number!"));
    }

    sorted_numbers.sort();

    let mut sum: u32 = 0;

    if n % 2 == 0 {
        for i in n/2..n {
            sum += sorted_numbers[i as usize];
        }
    } else {
        /* etc 5 3 2 1 1.
        sum (5+1)/2 = 3 numbers
        start loop at 3-1=2 because index 
        starts at 0
        then gathers positions 2, 3, 4
        */
        for i in (n+1)/2-1..n {
            sum += sorted_numbers[i as usize];
        }
    }

    println!("{}", sum)
}