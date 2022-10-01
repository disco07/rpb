#[allow(unused_mut)]
fn main() {
    let p = Person::new(String::from("Drissa"), 31);
    println!("{}", p.say_hello())
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn say_hello(&self) -> String {
        format!("Hello, I am {} and I have {}", &self.name, &self.age)
    }
}