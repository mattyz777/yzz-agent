use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_input(frame: &mut Frame, area: Rect, input: &str, cursor_position: usize) {
    let input_widget = Paragraph::new(input)
        .block(Block::default().borders(Borders::ALL).title("Input"));

    frame.render_widget(input_widget, area);

    // show cursor
    let cursor_x = area.x + 1 + cursor_position as u16;
    let cursor_y = area.y + 1;

    frame.set_cursor_position((cursor_x, cursor_y));
}