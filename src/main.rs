mod app;
mod command;
mod ui;
mod utils;

use std::io;

use app::App;
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind}, execute, terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use ui::{
    banner::draw_banner,
    footer::render_footer,
    menu::render_menu,
    layout::calculate_layout,
    input::render_input,
};

fn draw(frame: &mut ratatui::Frame, app: &App) {
    let layout = calculate_layout(frame);

    draw_banner(frame, layout.banner);
    render_menu(frame, layout.content, app);
    render_input(frame, layout.input, &app.input);
    render_footer(frame, layout.footer);
}

fn main() -> Result<()> {
    // Boilerplate: better error reporting
    color_eyre::install()?;

    // Boilerplate: read keyboard events without waiting for Enter
    enable_raw_mode()?;
    
    let mut stdout = io::stdout();
    
    // Boilerplate: restore original terminal content when the app exits
    execute!(stdout, EnterAlternateScreen)?;

    // Boilerplate: init
    let backend = CrosstermBackend::new(stdout);
    
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        // Boilerplate: render UI every loop iteration
        terminal.draw(|frame| {
            draw(frame, &app);
        })?;
        
        // Boilerplate: handle keyboard input events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // crossterm known issue, skip release events
                if key.kind != KeyEventKind::Press {
                    continue;  
                }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) => {
                        app.input.push(c);
                        app.show_commands = app.input.starts_with('/');
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                        app.show_commands = app.input.starts_with('/');
                    }
                    KeyCode::Enter => {
                        // todo : handle commands
                        app.input.clear();
                        app.show_commands = false;
                    }
                    _ => {}
                }
            }
        }
    }

    // Boilerplate: restore terminal state before exit
    disable_raw_mode()?;
    
    // Boilerplate: leave alternate screen and return to normal terminal
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}