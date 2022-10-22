#[derive(Clone)]
pub struct Spinner {
    frames: Vec<&'static str>,
}

impl Spinner {
    pub fn new(frames: Vec<&'static str>) -> Spinner {
        Self { frames }
    }

    pub fn spinning_cursor(&self, elapsed_time: f32) -> String {
        let iter = (elapsed_time * 4_f32) / (30.0 / 1000.0);
        let frame = self.frames.get(iter as usize % self.frames.len()).unwrap();
        frame.to_string()
    }
}
