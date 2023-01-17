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

fn recursive_dynamic_fibo(n: u32) {
    let n: usize = n.try_into().unwrap();
    let fibo_vec: Vec<usize> = vec![0, 1];

    iterator(n, fibo_vec);
}

fn iterator(n: usize, mut fibo_vec: Vec<usize>) -> usize {
    if fibo_vec.get(n) != None {
        println!("{}", fibo_vec[n]);
        return fibo_vec[n];
    } else {
        fibo_vec.push(iterator(n-1, fibo_vec));
        return fibo_vec[n];
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

        // println!("{}", simple_fibo(number));
        // println!("{}", recursive_fibo(number));
        // println!("{}", array_fibo(number));
        recursive_dynamic_fibo(number);
        break;
    }
}