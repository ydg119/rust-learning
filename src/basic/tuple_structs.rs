use std::fmt::write;

#[derive(Debug)]
struct Rectangle(i32, i32);

impl Rectangle {
    fn area(&self) -> i32 {
        self.0 * self.1
    }
}
pub fn test() {
    let r1 = Rectangle(15, 15);
    let area = r1.area();
    println!("area is: {area}");
}
