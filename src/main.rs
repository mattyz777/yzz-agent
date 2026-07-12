mod action;
mod app;
mod command;
mod event;
mod handler;
mod ui;
mod utils;

use std::io;

use app::App;
use color_eyre::Result;
use crossterm::{
    cursor::SetCursorStyle,
    execute, terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use ui::{
    banner::render_banner,
    footer::render_footer,
    input::render_input,
    menu::render_menu,
};

use crate::ui::layout::calculate_layout;


fn draw(frame: &mut ratatui::Frame, app: &App) {
    let layout = calculate_layout(frame, &app.input);

    render_banner(frame, layout.banner);
    render_menu(frame, layout.content, app);
    render_input(frame, layout.input, layout.input_hint, &app.input, app.cursor_position);
    render_footer(frame, layout.footer);
}

fn main() -> Result<()> {
    // Boilerplate: better error reporting
    color_eyre::install()?;

    // Boilerplate: read keyboard events without waiting for Enter
    enable_raw_mode()?;
    
    let mut stdout = io::stdout();
    
    // Boilerplate: restore original terminal content when the app exits
    execute!(stdout, SetCursorStyle::SteadyBlock, EnterAlternateScreen)?;

    // Boilerplate: init
    let backend = CrosstermBackend::new(stdout);

    // Boilerplate: init
    let mut terminal = Terminal::new(backend)?;

    // Boilerplate: init
    let mut app = App::new();

    loop {
        terminal.draw(|frame| draw(frame, &app))?;

        if let Some(action) = event::handle_event() {
            if !handler::handle_action(&mut app, action) {
                break;
            }
        }
    }

    // Boilerplate: restore terminal state before exit
    disable_raw_mode()?;
    
    // Boilerplate: leave alternate screen and return to normal terminal
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    
    Ok(())
}