mod timer;

use std::{io::{stdout, Result}, time::Duration};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand
    }, prelude::CrosstermBackend, style::{Color, Style}, widgets::Paragraph, Frame, Terminal
};
use timer::{Timer, TimerState};

pub fn run() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut timer = Timer::new();
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|frame| draw_ui(frame, &timer))?;
        handle_events(&mut should_quit, &mut timer)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn draw_ui(frame: &mut Frame, timer: &Timer) {
    let area = frame.size();
    let elapsed = timer.elapsed();
    let text = format!(
        "Welcome to CUBE\nTime: {:02}:{:02}:{:02}",
        elapsed.as_secs() / 60,
        elapsed.as_secs() % 60,
        elapsed.subsec_millis(),
    );
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Blue));

    frame.render_widget(paragraph, area);
}

fn handle_events(should_quit: &mut bool, timer: &mut Timer) -> Result<()>  {
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(' ') => {
                        match &timer.state() {
                            TimerState::Running { .. } => timer.pause(),
                            _ => {
                                timer.reset();
                                timer.start();
                            }
                        }
                    }
                    KeyCode::Char('q') => *should_quit = true,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}