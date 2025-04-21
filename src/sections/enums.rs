#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        // Don't need explicit dereference in newer versions of Rust
        match *self {
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Rectangle(w, h) => 2.0 * w + 2.0 * h,
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

enum Command {
    Clear,
    DrawLine(f64, f64),
    DrawShape(Shape),
}

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64),
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Location - unknown"),
            Location::Anonymous => println!("Location - anonymous"),
            Location::Known(lat, long) => println!("Location - {} {}", lat, long),
        }
    }
}

pub fn with_enums() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my shape is {:?}", my_shape);
    let _instruction = Command::Clear;

    match my_shape {
        Shape::Circle(r) => println!("Circle of radius {}", r),
        Shape::Rectangle(w, h) => println!("Rectangle width {} and height {}", w, h),
        Shape::Triangle(s1, s2, s3) => println!("Triangle w {}, {}, {}", s1, s2, s3),
    }

    let my_number = 1u8;

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        // Wildcard _ pattern must come at the end
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);
}
