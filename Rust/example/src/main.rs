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
    let coin = Coin::Quarter(UsState::Ohio);
    println!("{}", value_in_cents(coin));
}