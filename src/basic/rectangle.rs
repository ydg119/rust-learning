#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn test() {
    let rect1 = Rectangle {
        width: 35,
        height: 30,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1 area is {}", rect1.area());

    let square1 = Rectangle::square(5);
    println!("square1 is {:?}", square1);
    println!("square1 area is {}", square1.area());
}
