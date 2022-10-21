#[derive(Clone)]
pub struct Spinner {
    frames: Vec<&'static str>,
}

impl Spinner {
    pub fn new(frames: Vec<&'static str>) -> Spinner {
        Self { frames }
    }

    pub fn spinning_cursor(&self, index: usize) -> String {
        let iter = (2*index)%self.frames.len();
        let frame = self.frames.get(iter as usize).unwrap();
        frame.to_string()
    }
}
