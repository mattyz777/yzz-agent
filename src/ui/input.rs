use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_input(frame: &mut Frame, area: Rect, input: &str) {
    let input_widget = Paragraph::new(input)
        .block(Block::default().borders(Borders::ALL).title("Input"));

    frame.render_widget(input_widget, area);
}