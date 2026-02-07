
use std::io::Write;

fn main() {
    print!("Hello, ");
	std::io::stdout().flush().unwrap();
    eprintln!("An error occurred: invalid input");
    let name = "John";
    let age = 30;
    let message = format!("My name is {} and I am {} years old",name,age);
    println!("{}",message);
    println!("Hello, World!");

    let path = r"C:\Users\John\Documents\file.txt";
    println!("The file is located at: {}", path);

    let msg = format!(
        "I am {name} and I am {age} years old"
    );

    println!("{}", msg);

}