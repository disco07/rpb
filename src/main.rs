
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

fn main() {
    let name = String::from("Drissa");
    let p = Person::new(name, 31);
    println!("{}", p.say_hello())
}