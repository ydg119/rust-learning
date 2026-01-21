pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod innermod {
    use super::*;

    #[test]
    fn test() {
        let result = add(1, 2);
        assert_eq!(result, 3);
    }

    // #[test]
    // fn test_fail() {
    //     panic!("this test should fail");
    // }

    #[test]
    fn test_can_hold() {
        let larger = Rectangle {
            width: 10,
            height: 8,
        };
        let smaller = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger))
    }
}
