use rpb::bar::Bar;
use std::thread::sleep;
use std::time;
use rpb::styles::Themes;

fn main() {
    let mut bar = Bar::new(100);
    let mut bar2 = Bar::new(100);
    let mut bar3 = Bar::new(100);
    bar.set_theme(Themes::Small);
    bar2.set_theme(Themes::ColoredSmall);
    bar3.set_theme(Themes::ColoredMedium);
    for _i in 0..100 {
        bar.add(1);
        bar2.add(1);
        bar3.add(1);
        sleep(time::Duration::from_millis(100))
    }
}