struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle1 = Rectangle{ width: 30, height: 50, };
    let rectangle2 = Rectangle{ width: 31, height: 45, };

    println!(
        "사각형 1의 면적 : {} 제곱 픽셀",
        get_area(&rectangle1)
    );
    println!(
        "사각형 2의 면적 : {} 제곱 픽셀",
        get_area(&rectangle2)
    );
}

fn get_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}