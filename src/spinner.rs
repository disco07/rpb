pub struct Spinner {
    frames: Vec<&'static str>,
    interval: u16,
}

impl Spinner {
    pub fn new(frames: Vec<&str>, interval: u16) -> Spinner {
        Self {
            frames,
            interval
        }
    }

    pub fn spinning_cursor(&self, index: usize) -> String {
        let frame = self.frames.get(index%self.frames.len()).unwrap();
        frame.to_string()
    }
}


