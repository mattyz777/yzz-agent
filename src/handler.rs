use crate::action::Action;
use crate::app::App;
use crate::command::COMMANDS;


pub fn handle_action(app: &mut App, action: Action) -> bool {
    match action {
        Action::Quit => return false,
        Action::InputChar(c) => {
            app.input.insert(app.cursor_position, c);
            app.cursor_position += 1;
            update_command_list_state(app);
        }
        Action::Backspace => {
            if app.cursor_position > 0 {
                app.input.remove(app.cursor_position - 1);
                app.cursor_position -= 1;
            }
            update_command_list_state(app);
        }
        Action::MoveLeft => {
            if app.cursor_position > 0 {
                app.cursor_position -= 1;
            }
        }
        Action::MoveRight => {
            if app.cursor_position < app.input.len() {
                app.cursor_position += 1;
            }
        }
        Action::NewLine => {
            app.input.insert(app.cursor_position, '\n');
            app.cursor_position += 1;
        }
        Action::Escape => {
            if app.command_list_visible {
                app.input.clear();
                app.cursor_position = 0;
                app.command_list_visible = false;
                app.command_selected_index = 0;
                app.command_scroll_offset = 0;
            }
        }
        Action::Enter => {
            if app.command_list_visible {
                let filtered: Vec<_> = COMMANDS
                    .iter()
                    .filter(|cmd| cmd.name.starts_with(&app.input))
                    .collect();

                if let Some(cmd) = filtered.get(app.command_selected_index) {
                    app.input = cmd.name.to_string();
                    app.cursor_position = app.input.len();
                }
                app.command_list_visible = false;
                app.command_selected_index = 0;
                app.command_scroll_offset = 0;
            } else {
                // TODO: submit input
                app.input.clear();
                app.cursor_position = 0;
            }
        }
        Action::MoveUp => {
            if app.command_list_visible {
                if app.command_selected_index > 0 {
                    app.command_selected_index -= 1;

                    if app.command_selected_index < app.command_scroll_offset {
                        app.command_scroll_offset = app.command_selected_index;
                    }
                }
            }
        }
        Action::MoveDown => {
            if app.command_list_visible {
                let filtered: Vec<_> = COMMANDS
                    .iter()
                    .filter(|cmd| cmd.name.starts_with(&app.input))
                    .collect();

                if app.command_selected_index + 1 < filtered.len() {
                    app.command_selected_index += 1;
                    if app.command_selected_index >= app.command_scroll_offset + 8 {
                        app.command_scroll_offset += 1;
                    }
                }
            }
        }
    }
    true
}

fn update_command_list_state(app: &mut App) {
    app.command_list_visible = app.input.starts_with('/');
    if app.command_list_visible {
        app.command_selected_index = 0;
        app.command_scroll_offset = 0;
    }
}