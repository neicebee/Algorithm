#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let square1 = Rectangle::square(4);
    println!(
        "width: {}\nheight: {}", 
        square1.width, square1.height
    );
}