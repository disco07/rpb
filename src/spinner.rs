pub struct Spinner {
    frames: Vec<&'static str>,
}

impl Spinner {
    pub fn new(frames: Vec<&'static str>) -> Spinner {
        Self { frames }
    }

    pub fn spinning_cursor(&self, index: usize) -> String {
        let frame = self.frames.get(index % self.frames.len()).unwrap();
        frame.to_string()
    }
}
