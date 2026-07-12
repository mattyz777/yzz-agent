use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::action::Action;

pub fn handle_event() -> Option<Action> {
    if event::poll(std::time::Duration::from_millis(100)).ok()? {
        if let Event::Key(key) = event::read().ok()? {
            
            // crossterm known issue, ignore key release events
            if key.kind != KeyEventKind::Press {
                return None;
            }
            
            let action = match key.code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Char(c) => Action::InputChar(c),
                KeyCode::Backspace => Action::Backspace,
                KeyCode::Left => Action::MoveLeft,
                KeyCode::Right => Action::MoveRight,
                KeyCode::Up => Action::MoveUp,
                KeyCode::Down => Action::MoveDown,
                KeyCode::Esc => Action::Escape,
                KeyCode::Enter => {
                    if key.modifiers.contains(KeyModifiers::SHIFT) {
                        Action::NewLine
                    } else {
                        Action::Enter
                    }
                }
                _ => return None,
            };

            return Some(action);
        }
    }
    None
}