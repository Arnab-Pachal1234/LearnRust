use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Hello Rust Developer  Guess the Number : ");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You Guessed : {guess}");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=>{
                println!("Print enter valid input");
                continue;
            },
        }; //shadowing 

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {println!("You win");break;}
        }
    }
}
