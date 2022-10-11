use rpb::bar::Bar;
use std::thread::sleep;
use std::time;
use rpb::styles::Themes;

fn main() {
    let mut bar = Bar::new(100);
    bar.set_theme(Themes::ColoredSmall);
    for _i in 0..100 {
        bar.add(1);
        sleep(time::Duration::from_millis(100))
    }
}
