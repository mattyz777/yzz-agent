use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

pub struct LayoutAreas{
    pub banner: Rect,
    pub content: Rect,
    pub input: Rect,
    pub footer: Rect,
}

pub fn calculate_layout(frame: &Frame) -> LayoutAreas {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Length(8),   // banner
        Constraint::Min(0),      // content, flexible
        Constraint::Length(3),   // input
        Constraint::Length(1),   // footer
    ])
    .split(area);

    LayoutAreas {
        banner: chunks[0],
        content: chunks[1],
        input: chunks[2],
        footer: chunks[3],
    }
}