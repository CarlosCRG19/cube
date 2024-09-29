mod app;
mod math;
mod scramble;
mod session;
mod storage;
mod solve;
mod timer;
mod ui;

use crate::app::App;
use crate::storage::FileSystemStorage;
use crate::solve::SolveError;

use std::{result, io::{Stdout, stdout}, time::Duration};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
    }, prelude::*, Terminal
};
use thiserror::Error;
use std::io;

type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Error, Debug)]
pub enum CubeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Solve error: {0}")]
    InvalidSolve(#[from] SolveError),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error)
}

pub type Result<T> = result::Result<T, CubeError>;

pub fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    let storage = FileSystemStorage::build()?;
    let app = App::build(Box::new(storage))?;

    run_app(app, &mut terminal)?;

    restore_terminal(&mut terminal)?;

    Ok(())
}

fn setup_terminal() -> Result<Tui> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}


fn run_app(mut app: App, terminal: &mut Tui) -> Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(event) = event::read()? {
                if event.kind == KeyEventKind::Press {
                    app.on_key_pressed(event.code)?
                }
            }
        }
    }
    Ok(())
}

fn restore_terminal(terminal: &mut Tui) -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
