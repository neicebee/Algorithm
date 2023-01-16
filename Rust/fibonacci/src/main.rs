use std::io;

fn simple_fibo(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;

    if n <= 1 {
        return n.into();
    }

    for _ in 0..n-1 {
        let tmp = a;
        a = b;
        b = tmp+b;
    }
    return b;
}

fn recursive_fibo(n: u32) -> u64 {
    if n <= 1 {
        return n.into();
    } else {
        return recursive_fibo(n-1) + recursive_fibo(n-2);
    }
}

fn main() {
    loop {
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Unable to read input!");
        
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error! You have to input number!");
                continue
            },
        };

        println!("{}", simple_fibo(number));
        println!("{}", recursive_fibo(number));
        break;
    }
}