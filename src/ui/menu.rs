use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn render_menu(frame: &mut Frame, area: Rect, _app: &App) {
    let items = vec![
        ListItem::new("Review Code"),
        ListItem::new("Generate Commit"),
        ListItem::new("Settings"),
    ];

    let list = List::new(items)
        .block(Block::default().title("Menu").borders(Borders::ALL));

    frame.render_widget(list, area);
}