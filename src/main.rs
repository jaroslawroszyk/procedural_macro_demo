use facet::Facet;
// src/main.rs
use procedural_macro::{Repeat};

trait Repeat {
    fn repeat(&self);
}

#[derive(Repeat)]
#[count = 5]
struct MyStruct;

#[derive(Facet, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let s = MyStruct;
    s.repeat();

    let shape = Person::SHAPE;
    println!("Shape: {:?}", shape);
}