use ratatui::{
    layout::Rect,
    widgets::Paragraph,
    Frame,
};

pub fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("");
    frame.render_widget(footer, area);
}