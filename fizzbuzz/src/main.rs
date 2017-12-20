use std::io;

fn explain_app(){
    println!("FizzBuzz checker application");
    println!("Check fizz or buzz in your input value");
}

fn order_to_user(){
    println!("Input your value I check!");
}


fn main() {
    explain_app();
    order_to_user();

    let mut user_value = String::new();

    io::stdin().read_line(&mut user_value)
        .expect("Failed to read line");

    // Covert string to Integer
    let _input: u32 = user_value.trim().parse().expect("Wrong value");

    if _input % 3 == 0 {
        println!("Fizz");
    } else if _input % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", _input);
    }
}
