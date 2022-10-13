use rpb::bar::Bar;
use rpb::styles::Themes;
use std::thread::sleep;
use std::time;

fn main() {
    let mut bar = Bar::new(10);
    let mut bar2 = Bar::new(50);
    let mut bar3 = Bar::new(500);
    bar.set_theme(Themes::Small);
    bar2.set_theme(Themes::ColoredSmall);
    bar3.set_theme(Themes::ColoredSmall);
    bar2.set_position(1);
    bar3.set_position(2);
    for _i in 0..10 {
        bar.add(1);
        for _i in 0..5 {
            bar2.add(1);
            for _i in 0..10 {
                bar3.add(1);
                sleep(time::Duration::from_millis(50))
            }
        }
    }
    print!("\n");
    print!("\n");
    print!("\n");
}
