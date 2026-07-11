use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub fn render_input_hint(frame: &mut Frame, area: Rect, input: &str) {
    if input.is_empty() {
        let hint = Paragraph::new("/copy to clipboard")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Right);

        frame.render_widget(hint, area);
    }
}