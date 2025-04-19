struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    fn new(w: f64, h: f64) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }

    fn new_2(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}
