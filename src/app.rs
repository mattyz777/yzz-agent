pub struct App {
    pub selected: usize,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected: 0,
            running: true,
        }
    }
}