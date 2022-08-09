fn main(){
    loop{
        let mut inputnums = String::new();

        std::io::stdin().read_line(&mut inputnums)
            .expect("nums reading error!");

        let splitnums: Vec<&str> = inputnums.split(' ')
            .collect();

        let a = &splitnums[0];
        let b = &splitnums[1];

        let a : u32 = match a.trim().parse(){
            Ok(x) => x,
            Err(_) => continue,
        };

        let b : u32 = match b.trim().parse(){
            Ok(x) => x,
            Err(_) => continue,
        };

        println!("{}", a+b);
    }
}