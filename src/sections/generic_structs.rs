pub struct Rectangle<T> {
    width: T,
    height: T,
}

pub struct HeteroRectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> HeteroRectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width // use a reference, don't know if T will be on heap or stack
    }
}

// Implementation for specific types
impl HeteroRectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

pub fn do_something_w_generic() {
    let int_rect = Rectangle {
        width: 1,
        height: 3,
    };

    let float_rect = Rectangle {
        width: 1.0,
        height: 3.0,
    };

    let u8_rect = Rectangle {
        width: 1u8,
        height: 3u8,
    };

    let hetero_rectangle = HeteroRectangle {
        width: 1u8,
        height: 3u16,
    };
}
