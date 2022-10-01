use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::time;

struct Bar {
    state: State,
    options: Option,
    theme: Theme
}

struct State {
    percent: f64,
    current: i64,
    current_graph_rate: isize,
}

struct Option {
    total: i64,
    start_time: time::Time,
}

struct Theme {
    rate: String,
    bar_type: String,
    bar_start: String,
    bar_end: String,
    bar_width: String,

}

fn main() {
    let name = Rc::new(String::from("Drissa"));
    let p = Person::new(name, 31);
    println!("{}", p.say_hello());
    let add = |x, y| x > y;
    let mut f = File::create("src/t.txt").unwrap();
    f.write_all("bonjour".as_bytes()).unwrap();
    // let data = File::open("/src/t.txt").unwrap();

    println!("{}", add(1.5, 2.5));
}