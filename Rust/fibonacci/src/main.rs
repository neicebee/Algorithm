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

fn array_fibo(n: u32) -> usize {
    let mut fibo_arr: [usize; 2] = [0, 1];
    let n: usize = n.try_into().unwrap();

    for i in fibo_arr.len()..n+1 {
        fibo_arr[i%2] = fibo_arr[(n-1)%2] + fibo_arr[(n-2)%2];
    }

    return fibo_arr[n%2]
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

        println!("Simple Fibonacci Sequence: {}", simple_fibo(number));
        println!("Recursive Fibonacci Sequence: {}", recursive_fibo(number));
        println!("Array Fibonacci Sequence: {}", array_fibo(number));
        break;
    }
}