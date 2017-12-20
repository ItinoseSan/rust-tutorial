use std::io;

fn explain_app(){
    println!("FizzBuzz checker application");
    println!("Check fizz or buzz in your input value");
}

fn order_to_user(){
    println!("Input your value I check!");
}

fn checker(value: u32){
    match (value % 3, value % 5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        (_, _) => println!("{}",value),
    }
}

fn main() {
    explain_app();
    order_to_user();

    let mut user_value = String::new();

    io::stdin().read_line(&mut user_value)
        .expect("Failed to read line");

    // Covert string to Integer
    let _input: u32 = user_value.trim().parse().expect("Wrong value");

    checker(_input);
}


