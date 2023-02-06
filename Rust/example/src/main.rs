#[derive(Debug)]
enum UsState {
    Alabama, Alaska, Arizona,
    Arkansas, California, Colorado,
    Florida, Kentucky, Ohio,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        },
    }
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}.", state);
    } else {
        count += 1;
        println!("{}", count);
    }
}