use std::f32::consts::PI;

pub trait Calculate {
    fn area(&self) -> f32;
}

pub fn print_area<T: Calculate>(graph: &T) -> f32 {
    println!("Area: {}", graph.area());
    graph.area()
}

struct Circle {
    r: f32,
}

impl Calculate for Circle {
    fn area(&self) -> f32 {
        self.r * self.r * PI
    }
}

struct Rectangle {
    w: f32,
    h: f32,
}

impl Calculate for Rectangle {
    fn area(&self) -> f32 {
        self.w * self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { r: 2.0 };
        assert_eq!(circle.area(), 2.0 * 2.0 * PI);
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle { w: 10.0, h: 2.5 };
        assert_eq!(rectangle.area(), 10.0 * 2.5)
    }
}
