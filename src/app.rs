pub struct App {
    pub input: String,
    pub show_commands: bool,
    pub last_message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            show_commands: false,
            last_message: String::new(),
        }
    }
}