use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess the number!");

    let secret_num = rand::thread_rng().gen_range(1, 51);

    println!("The secret number is {}", secret_num);

    loop {

        println!("Please input your guess");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to readline");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guesse: {}", guess);
        
        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("you win!");
                break;
            },
            Ordering::Greater => println!("down!"),
            Ordering::Less => println!("up!")
        }
    }
}

