use rpb::bar::Bar;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let source = File::open("indicateurs_v2.sql")?;
    let mut target = File::create("src.sql")?;
    let bar = Bar::default_bytes(source.metadata()?.len() as i64, "downloading");
    io::copy(&mut bar.reader(source), &mut target).unwrap();
    Ok(())
}
