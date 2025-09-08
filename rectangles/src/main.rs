#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

use std::fmt;

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rectangle {{ width: {}, height: {}, area: {}, perimeter: {} }}",
            self.width,
            self.height,
            self.area(),
            self.perimeter()
        )
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(20, 30);

    println!("Rectangle is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("Square {}", Rectangle::square(30));

    let one = 2;
    let two: Option<i32> = Some(2);
    let two = two.unwrap_or(3);

    println!("{}", one * two);
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn area_computes() {
        let r = Rectangle::new(3, 4);
        assert_eq!(r.area(), 12);
    }

    #[test]
    fn perimeter_computes() {
        let r = Rectangle::new(3, 4);
        assert_eq!(r.perimeter(), 14);
    }

    #[test]
    fn can_hold_true_when_strictly_greater() {
        let big = Rectangle::new(5, 6);
        let small = Rectangle::new(4, 5);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }

    #[test]
    fn square_constructor_and_is_square() {
        let s = Rectangle::square(10);
        assert!(s.is_square());
        assert_eq!(s.width, 10);
        assert_eq!(s.height, 10);
    }
}
