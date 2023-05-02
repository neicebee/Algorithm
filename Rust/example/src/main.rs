#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn struct_mix<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 4.5, y: 7 };
    let p2 = Point { x: "Hello", y: "world!" };

    println!("{:?}", p1.struct_mix(p2));
}