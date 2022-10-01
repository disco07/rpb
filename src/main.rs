#[allow(unused_mut)]
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    name: Rc<String>,
    age: u32,
}

impl Person {
    fn new(name: Rc<String>, age: u32) -> Self {
        Self { name, age }
    }

    fn say_hello(&self) -> String {
        format!("Hello, I am {} and I have {}", &self.name, &self.age)
    }
}

fn main() {
    let rc = Rc::new(String::from("Drissa"));
    let p = Person::new(rc, 31);
    println!("{}", p.say_hello())
}