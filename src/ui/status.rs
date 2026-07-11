use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_status(frame: &mut Frame) {
    let area = frame.area();

    let layout = Layout::vertical([
        Constraint::Length(15),
        Constraint::Min(0),
    ])
    .split(area);

    let status = Paragraph::new("Ready")
        .block(Block::default().title("Status").borders(Borders::ALL));

    frame.render_widget(status, layout[1]);
}