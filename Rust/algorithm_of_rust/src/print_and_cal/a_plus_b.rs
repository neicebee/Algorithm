pub fn plus(){
    let mut input_nums = String::new();

    std::io::stdin().read_line(&mut input_nums)
        .expect("nums reading error!");

    let mut split_nums: Vec<&str> = input_nums.split(' ')
        .collect();

    let mut a = &split_nums[0];
    let mut b = &split_nums[1];

    println!("{}", a+b);
}