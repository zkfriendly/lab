use std::io;
use rand::Rng;

fn main() {
    let mut count = String::new();

    println!("Count: ");

    io::stdin().read_line(&mut count).expect("Could not get user input!");

    let count: usize = count.trim().parse().expect("Invalid Number");

    let mut numbers: Vec<usize> = Vec::new();
    
    
    for _ in 0..count {
        let number = rand::thread_rng().gen_range(1..=100);
        numbers.push(number);
    }
    
    print_vec(&numbers);

    sort(&mut numbers);

    print_vec(&numbers);
} 

fn print_vec(numbers: &Vec<usize>) {
    for (i, number) in numbers.iter().enumerate() {
        println!("{i} - {number}");
    }
}

fn sort(numbers: &mut Vec<usize>) {
    for i in 0..numbers.len() - 1 {
        for j in i+1..numbers.len() {
            if numbers[j] <= numbers[i] {
                let tmp = numbers[i];
                numbers[i] = numbers[j];
                numbers[j] = tmp;
            }
        }
    }
}