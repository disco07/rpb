use rpb::bar::Bar;
use rpb::styles::Themes;
use std::thread::sleep;
use std::time;

fn main() {
    let mut bar = Bar::new(100);
    let mut bar2 = Bar::new(100);
    let mut bar3 = Bar::new(100);
    let mut bar4 = Bar::new(100);


    bar.set_theme(Themes::Basic);
    bar.set_description("bar1");

    bar2.set_theme(Themes::Small);
    bar2.set_description("bar2");

    bar3.set_theme(Themes::ColoredSmall);
    bar3.set_description("bar3");

    bar4.set_theme(Themes::ColoredMedium);
    bar4.set_description("bar4");

    bar2.set_position(1);
    bar3.set_position(2);
    bar4.set_position(3);

    for _i in 0..100 {
        bar.add(1);
        bar2.add(1);
        bar3.add(1);
        bar4.add(1);
        sleep(time::Duration::from_millis(50))
    }
    eprint!("{}", "\n".repeat(4));
}
