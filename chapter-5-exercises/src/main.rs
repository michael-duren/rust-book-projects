#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        &self.area() > &rect2.area()
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle::square(30);

    println!("rect one is {:?}", rect);
    println!("Area is {}", rect.area());

    let rect2 = Rectangle {
        width: 60,
        height: 60,
    };

    println!("rect one cannot hold rect two: {}", rect.can_hold(&rect2));
}
