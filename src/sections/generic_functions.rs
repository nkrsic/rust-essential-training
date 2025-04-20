// Can only use types that implement the
// trait PartialOrd (in prelude)
pub fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
