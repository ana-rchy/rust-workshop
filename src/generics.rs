use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

enum WorseOption<T, U> {
    Some(T),
    Maybe(U),
    None,
}

// only implemented when T in 'Point' is an f32
impl Point<f32> {
    fn half(self) -> Self {
        Point {
            x: self.x / 2.0,
            y: self.y / 2.0,
        }
    }
}

// implemented for 'Point' as a whole, with T as a generic
impl<T> Point<T> {
    fn swap(self) -> Self {
        Point {
            x: self.y,
            y: self.x,
        }
    }
}

fn generics() {
    print_thing(&"Man I'm so hungry...");
    print_thing(&2);
}

// 'Display' is another trait, meaning anything that can be printed in a non-debug way
// 'Debug' is for like display but for debug
fn print_thing<T: Display>(thing: &T) {
    println!("{thing}");
}
