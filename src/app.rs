pub struct App {
    pub input: String,
    pub cursor_position: usize,
    pub command_list_visible: bool,
    pub command_selected_index: usize,
    pub command_scroll_offset: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            command_list_visible: false,
            command_selected_index: 0,
            command_scroll_offset: 0,
        }
    }
}