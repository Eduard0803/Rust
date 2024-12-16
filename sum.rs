use std::io;
use std::io::Write;


fn print(output: &str) {
    print!("{}", output);
    io::stdout().flush().expect("Failed to flush stdout");
}

fn main(){
    print("Enter a first number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n1: i32 = input.trim().parse().expect("Please enter a valid number!");

    print("Enter a second number: ");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n2: i32 = input.trim().parse().expect("Please enter a valid number!");

    println!("SUM = {}", n1 + n2);
}

