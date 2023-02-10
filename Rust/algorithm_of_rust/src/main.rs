use std::io;
const Z: [u32; 3] = [10, 100, 1000];

fn main() {
    let mut tmp: u32 = 0;
    let mut a = String::new();
    let mut b = String::new();
    io::stdin().read_line(&mut a)
        .expect("Error");
    io::stdin().read_line(&mut b)
        .expect("Error");
    let mul_nums: [u32; 2] = [
        a.trim().parse().expect("Error"),
        b.trim().parse().expect("Error")
    ];

    for i in Z {
        println!(
            "{}",
            mul_nums[0]*((mul_nums[1]%i)-tmp)
        );
        tmp = mul_nums[1]%i;
    }
}