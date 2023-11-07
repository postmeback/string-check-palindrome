use std::io;

fn main() {
    let mut org: String = String::new();

    println!("Enter original string");

    io::stdin().read_line(&mut org).expect("Wrong input");

    //org = org.trim().to_string();

    let reversed: String = org.chars().rev().collect(); 

    let result: bool = org == reversed;

    println!("The result is {}", result);
}
