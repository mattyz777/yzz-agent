use ratatui::{
    Frame, 
    layout::{Alignment, Rect}, 
    widgets::{Block, Paragraph}
};

pub fn draw_banner(frame: &mut Frame, area: Rect) {
    let banner = r#"Welcome to YZZ agent"#;

    let paragraph = Paragraph::new(format!(
        "{}\n\n{}",
        banner,
        "https://github.com/mattyz777/yzz-agent"
    ))
    .alignment(Alignment::Center)
    .block(Block::default());

    frame.render_widget(paragraph, area);
}