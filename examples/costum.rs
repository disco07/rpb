use rpb::bar::Bar;
use rpb::styles::Themes;
use std::thread::sleep;
use std::time;

fn main() {
    let mut bar = Bar::new(100);
    let mut bar2 = Bar::new(100);
    let mut bar3 = Bar::new(100);
    bar.set_theme(Themes::Small);
    bar2.set_theme(Themes::ColoredSmall);
    bar3.set_theme(Themes::ColoredMedium);
    bar2.set_position(2);
    bar3.set_position(4);
    for _i in 0..100 {
        bar.add(1);
        bar2.add(1);
        bar3.add(1);
        sleep(time::Duration::from_millis(100))
    }
}
