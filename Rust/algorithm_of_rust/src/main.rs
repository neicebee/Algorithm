use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut time: Vec<i32> = buf.trim().split(' ')
        .map(
            |x| x.parse().unwrap()
        ).collect();
    if time[1]-45 < 0 { 
        if time[0]-1 < 0 {
            time[0] = 23;
        } else {
            time[0] -= 1; 
        }
        time[1] = (time[1]+60)-45;
    } else {
        time[1] = time[1]-45;
    }
    println!("{} {}", time[0], time[1]);
}