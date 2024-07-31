mod timer;
mod math;


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
    let mut session = Session::new();

    while !should_quit {
        terminal.draw(|frame| draw_ui(frame, &timer, &session))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(' ') => {
                            match &timer.state() {
                                TimerState::Running { .. } => {
                                    timer.pause();
                                    session.save_time(timer.elapsed());
                                }
                                _ => {
                                    timer.reset();
                                    timer.start();
                                }
                            }
                        }
                        KeyCode::Char('q') => should_quit = true,
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn draw_ui(frame: &mut Frame, timer: &Timer, session: &Session) {
    let area = frame.size();
    let elapsed = timer.elapsed();
    let text = format!(
        "Welcome to CUBE\nTime: {}\nTotal times: {}\navg: {} (σ = {})",
        render_time(elapsed),
        session.times().len(),
        if let Some(avg) = math::avg(session.times()) { render_time(avg) } else { "DNF".to_string() },
        if let Some(std) = math::std(session.times()) { render_time(std) } else { "-1".to_string() },
    );
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Blue));

    frame.render_widget(paragraph, area);
}

fn render_time(time: Duration) -> String {
    format!("{:02}:{:02}:{02}", time.as_secs() / 60, time.as_secs() % 60, time.subsec_millis())
}

struct Session {
    times: Vec<Duration>,
}

impl Session {
    fn new() -> Self {
        Session { times: Vec::new() }
    }

    fn from_times(times: Vec<Duration>) -> Self {
        Session { times }
    }

    fn times(&self) -> &Vec<Duration> {
        &self.times
    }

    fn save_time(&mut self, time: Duration) {
        self.times.push(time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new_session() {
        let session = Session::new();
        assert_eq!(session.times().len(), 0);
    }

    #[test]
    fn test_new_session_from_times() {
        let session = Session::from_times(vec![
            Duration::from_millis(5440),
            Duration::from_millis(7480),
            Duration::from_millis(5400),
        ]);
        assert_eq!(session.times().len(), 3);
    }

    #[test]
    fn test_save_time() {
        let mut session = Session::new();
        session.save_time(Duration::from_millis(5440));
        session.save_time(Duration::from_millis(7480));
        session.save_time(Duration::from_millis(5400));
        assert_eq!(session.times().len(), 3);
    }
}