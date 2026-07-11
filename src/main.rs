mod app;
mod command;
mod ui;
mod utils;

use std::io;

use app::App;
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
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
};

fn draw(frame: &mut ratatui::Frame, app: &App) {
    let layout = calculate_layout(frame);

    draw_banner(frame, layout.banner);
    render_menu(frame, layout.content, app);
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

    let app = App::new();

    loop {
        // Boilerplate: render UI every loop iteration
        terminal.draw(|frame| {
            draw(frame, &app);
        })?;
        
        // Boilerplate: handle keyboard input events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // quit application by pressing 'q'
                if key.code == KeyCode::Char('q') {
                    break;
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