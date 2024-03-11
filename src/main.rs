use std::io;//prelude
use rand::Rng;//trait
use std::cmp::Ordering;


fn main(){
    println!("guessing game!");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
    println!("secret number is {}", secret_number);

    loop {
        println!("guess a number");


        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to guess");
        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("the number you guessed was {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"), //arm
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you are right");
                break;
            },
        }
    
    }
}