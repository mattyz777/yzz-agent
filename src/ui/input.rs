use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub fn render_input(
    frame: &mut Frame,
    input_area: Rect,
    hint_area: Rect,
    input: &str,
    cursor_position: usize,
) {
    let (text, text_color) = if input.is_empty() {
        ("ask a question or describe a task", Color::DarkGray)
    } else {
        (input, Color::White)
    };

    let input_widget = Paragraph::new(text)
        .style(Style::default().fg(text_color))
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(input_widget, input_area);

    if input.is_empty() {
        let hint = Paragraph::new("/copy to clipboard")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Right);
        frame.render_widget(hint, hint_area);
    }

    let (cursor_x, cursor_y) = calculate_cursor_position(input, cursor_position, input_area);
    frame.set_cursor_position((cursor_x, cursor_y));
}


fn calculate_cursor_position(input: &str, cursor_position: usize, area: Rect) -> (u16, u16) {
    if input.is_empty() {
        return (area.x, area.y);
    }

    let width = area.width as usize;
    let mut row = 0;
    let mut col = 0;
    let mut pos = 0;

    for ch in input.chars() {
        if pos == cursor_position {
            break;
        }

        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
            if col >= width {
                row += 1;
                col = 0;
            }
        }

        pos += 1;
    }

    if pos == cursor_position && cursor_position > 0 {
        let last_char = input.chars().nth(cursor_position - 1);
        if last_char == Some('\n') {
            row += 1;
            col = 0;
        }
    }

    (area.x + col as u16, area.y + row as u16)
}