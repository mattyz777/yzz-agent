use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

pub struct LayoutAreas{
    pub banner: Rect,
    pub content: Rect,
    pub input: Rect,
    pub input_hint: Rect,
    pub footer: Rect,
}

pub fn calculate_layout(frame: &Frame, input: &str) -> LayoutAreas {
    let area = frame.area();
    let width = area.width as usize;
    let input_lines = calculate_input_lines(input, width);
    let input_height = input_lines.clamp(1, (area.height * 2 / 3) as usize) as u16;

    let chunks = Layout::vertical([
        Constraint::Length(8),              // banner
        Constraint::Min(0),                 // content, flexible
        Constraint::Length(input_height),   // input (dynamic height)
        Constraint::Length(1),              // input hint
        Constraint::Length(1),              // footer (temp no footer)
    ])
    .split(area);

    LayoutAreas {
        banner: chunks[0],
        content: chunks[1],
        input: chunks[2],
        input_hint: chunks[3],
        footer: chunks[4],
    }
}


fn calculate_input_lines(input: &str, width: usize) -> usize {
    if input.is_empty() {
        return 1;
    }

    let mut lines = 1;
    let mut col = 0;

    for ch in input.chars() {
        if ch == '\n' {
            lines += 1;
            col = 0;
        } else {
            col += 1;
            if col >= width {
                lines += 1;
                col = 0;
            }
        }
    }

    lines
}