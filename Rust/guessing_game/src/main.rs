use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("The number of player thinks.");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Unable to read input!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Input Number: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Your number is Less."),
            Ordering::Greater => println!("Your number is Greater."),
            Ordering::Equal => {
                println!("Correct!!");
                break;
            }
        }
    }
}