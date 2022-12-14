use crate::bar::Bar;
use std::io;

#[derive(Debug)]
pub struct BarIter<T> {
    pub it: T,
    pub bar: Bar,
}

impl<R: io::Read> io::Read for BarIter<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.it.read(buf)?;
        self.bar.add(n);
        Ok(n)
    }
}

impl<W: io::Write> io::Write for BarIter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.it.write(buf)?;
        self.bar.add(n);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let n = self.it.write(buf)?;
        self.bar.add(n);
        Ok(())
    }
}
