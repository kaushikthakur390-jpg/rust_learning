use std::io;

fn main() {
    let result = number();

    println!("Returned value is {result}");
}

fn number(x: i32) -> i32 {
    println!("The argument passed to the function is {x}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let x: i32 = input
        .trim()
        .parse()
        .expect("Not a number");

    x
}