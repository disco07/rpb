use rpb::bar::Bar;
use std::thread::sleep;
use std::time;

fn main() {
    let mut bar = Bar::new(100);

    for _i in 0..100 {
        bar.add(1);
        sleep(time::Duration::from_millis(100))
    }
}
