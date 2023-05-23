use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("src/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for l in contents.lines().skip(1) {
        print!("{l}: ");
        let r = l.split(' ').map(
            |x| x.parse::<i32>().unwrap()
        ).sum::<i32>();
        println!("{r}");
    }
}