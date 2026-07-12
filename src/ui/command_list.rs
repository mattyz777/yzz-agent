use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::command::{COMMANDS, Command};

const MAX_VISIBLE: usize = 8;

pub fn render_command_list(
    frame: &mut Frame,
    area: Rect,
    input: &str,
    scroll_offset: usize,
    selected_index: usize,
) {
    if !input.starts_with('/') {
        return;
    }

    let filtered: Vec<&Command> = COMMANDS
        .iter()
        .filter(|cmd| cmd.name.starts_with(input))
        .collect();

    if filtered.is_empty() {
        return;
    }

    // 计算最大命令名长度用于对齐
    let max_name_len = filtered
        .iter()
        .map(|cmd| cmd.name.len())
        .max()
        .unwrap_or(0);

    for (i, cmd) in filtered
        .iter()
        .skip(scroll_offset)
        .take(MAX_VISIBLE)
        .enumerate()
    {
        let actual_index = scroll_offset + i;
        let is_selected = actual_index == selected_index;

        let name_color = if is_selected { Color::Blue } else { Color::White };
        let desc_color = if is_selected { Color::Gray } else { Color::DarkGray };

        // 固定宽度对齐
        let name_span = Span::styled(
            format!("{:width$}", cmd.name, width = max_name_len),
            Style::default().fg(name_color),
        );
        let desc_span = Span::styled(
            format!("    {}", cmd.description),
            Style::default().fg(desc_color),
        );
        let line = Line::from(vec![name_span, desc_span]);

        let y = area.y + i as u16;

        if y >= area.y + area.height {
            break;
        }

        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, Rect::new(area.x, y, area.width, 1));
    }

    // bottom (+N more) - 显示底部还有多少未显示
    let remaining = filtered.len().saturating_sub(scroll_offset + MAX_VISIBLE);
    if remaining > 0 {
        let more_text = format!("(+{} more)", remaining);
        let more_y = area.y + MAX_VISIBLE as u16;
        let more = Paragraph::new(more_text)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(more, Rect::new(area.x, more_y, area.width, 1));
    }
}