use std::io;

fn main() {

    let mut fibo_arr = [0, 1];

    loop {
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Unable to read input!");
        
        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error! You have to input number!");
                continue
            },
        };

        println!("{}", fibo_arr[number]);

        // if fibo_arr[number] != Err {
        //     println!("{}", fibo_arr[number]);
        // }
        break;
    }

}