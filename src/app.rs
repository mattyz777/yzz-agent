pub struct App {
    pub cursor_position: usize,
    pub input: String,
    pub last_message: String,
    pub show_commands: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            cursor_position: 0,
            input: String::new(),
            last_message: String::new(),
            show_commands: false,
        }
    }
}