// src/main.rs
use procedural_macro::Repeat;

trait Repeat {
    fn repeat(&self);
}

#[derive(Repeat)]
#[count = 5]
struct MyStruct;

fn main() {
    let s = MyStruct;
    s.repeat();
}
