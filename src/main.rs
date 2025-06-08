use facet::Facet;
// src/main.rs
use procedural_macro::{Foo, GenerateStruct, HelloWorld, Repeat};

// Example of GenerateStruct macro
#[derive(Debug, GenerateStruct)]
struct GeneratedUser {
    id: u32,
    name: String,
    active: bool,
}

trait HelloWorld {
    fn hello_world(&self);
}

// Example of HelloWorld macro
#[derive(HelloWorld)]
struct Greeter;

// Example of Foo macro
trait Foo {
    fn get_attributes(&self) -> Vec<&'static str>;
}

#[derive(Foo)]
#[foo = "bar"]
#[bar = "baz"]
struct MyFoo;

// Example of Repeat macro
trait Repeat {
    fn repeat(&self);
}

#[derive(Repeat)]
#[count = 3]
struct MyStruct;

// Example of Foo macro

// Example of Facet macro
#[derive(Facet, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Test GenerateStruct
    let user = GeneratedUser::new();
    println!("Generated user: {:?}", user);

    // Test HelloWorld
    let g = Greeter;
    g.hello_world();

    // Test Foo
    let f = MyFoo;
    println!("Foo attributes: {:?}", f.get_attributes());

    let s = MyStruct;
    s.repeat();

    // Test Facet
    let shape = Person::SHAPE;
    println!("Person shape: {:?}", shape);
}
