mod app;
mod timer;
mod math;
mod session;
mod scramble;
mod solve;
mod ui;

use crate::app::App;

use std::{io::{self, Stdout, stdout}, time::Duration};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
    }, prelude::*, Terminal
};

type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;

    let app = App::new();
    run_app(app, &mut terminal)?;

    restore_terminal(&mut terminal)?;

    Ok(())
}

fn setup_terminal() -> io::Result<Tui> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}


fn run_app(mut app: App, terminal: &mut Tui) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(event) = event::read()? {
                if event.kind == KeyEventKind::Press {
                    app.on_key_pressed(event.code)
                }
            }
        }
    }
    Ok(())
}

fn restore_terminal(terminal: &mut Tui) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
