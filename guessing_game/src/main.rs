use std::io;

fn main() {
    println!("guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to readline");

    println!("You guesse: {}", guess)
}
