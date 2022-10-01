use std::fs::File;
use std::io::Write;
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
    let name = Rc::new(String::from("Drissa"));
    let p = Person::new(name, 31);
    println!("{}", p.say_hello());
    let add = |x, y| {
      x > y
    };
    let mut f = File::create("src/t.txt").unwrap();
    f.write_all("bonjour".as_bytes()).unwrap();
    // let data = File::open("/src/t.txt").unwrap();

    println!("{}", add(1.5, 2.5));
}