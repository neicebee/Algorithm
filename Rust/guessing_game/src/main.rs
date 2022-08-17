use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The number of player must guess: {}", secret_number);
    println!("The number of player thinks.");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Unable to read input!");

    println!("Input Number: {}", guess);
}