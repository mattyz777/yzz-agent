use crate::action::Action;
use crate::app::App;

pub fn handle_action(app: &mut App, action: Action) -> bool {
    match action {
        Action::Quit => return false,
        Action::InputChar(c) => {
            app.input.insert(app.cursor_position, c);
            app.cursor_position += 1;
            app.show_commands = app.input.starts_with('/');
        }
        Action::Backspace => {
            if app.cursor_position > 0 {
                app.input.remove(app.cursor_position - 1);
                app.cursor_position -= 1;
            }
            app.show_commands = app.input.starts_with('/');
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
        Action::Enter => {
            // TODO: handle commands
            app.input.clear();
            app.cursor_position = 0;
            app.show_commands = false;
        }
    }
    true
}