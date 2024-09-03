mod timer;
mod math;
mod session;
mod scramble;
mod solve;

use std::{io::{self, Stdout, stdout}, time::Duration};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
    }, prelude::CrosstermBackend, style::{Color, Style}, widgets::Paragraph, Frame, Terminal
};
use solve::Solve;
use timer::{Timer, TimerState};
use session::Session;
use scramble::{Puzzle, Scrambler};

type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;

    let app = App::new();
    run_app(app, &mut terminal)?;

    restore_terminal(&mut terminal)?;

    Ok(())
}

fn setup_terminal() -> io::Result<Tui> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

struct App {
    should_quit: bool,
    timer: Timer,
    session: Session,
    current_scramble: String, 
}

impl App {
    fn new() -> App {
        App {
            timer: Timer::new(),
            should_quit: false,
            session: Session::new(),
            current_scramble: Scrambler::new_scramble(Puzzle::Cube3x3)
        }
    }
}

fn run_app(app: App, terminal: &mut Tui) -> io::Result<()> {
    let mut app = app;
    while !app.should_quit {
        terminal.draw(|frame| render(frame, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(' ') => {
                            match &app.timer.state() {
                                TimerState::Running { .. } => {
                                    let solve = Solve::build(app.current_scramble, Some(app.timer.elapsed()), None).unwrap();
                                    app.timer.pause();
                                    app.session.save_solve(solve);
                                    app.current_scramble = Scrambler::new_scramble(Puzzle::Cube3x3);
                                }
                                _ => {
                                    app.timer.reset();
                                    app.timer.start();
                                }
                            }
                        }
                        KeyCode::Char('q') => app.should_quit = true,
                        _ => {}
                    }
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


fn render(frame: &mut Frame, app: &App) {
    let area = frame.size();
    let elapsed = app.timer.elapsed();

    // [TODO]: handle averages that include a DNF time
    // [TODO]: perhaps `std::avg` should allow a slice of references to Duration
    // `Some(*time)` is a little workaround to this issue rn
    let times: Vec<_> = app.session.solves()
        .iter()
        .filter_map(|solve| {
            if let Some(time) = solve.time() {
                Some(*time)
            } else {
                None
            }
        })
        .collect();


    let text = format!(
        "Welcome to CUBE\nScramble: {}\nTime: {}\nTotal times: {}\navg: {} (σ = {})",
        app.current_scramble,
        render_time(elapsed),
        app.session.solves().len(),
        if let Some(avg) = math::avg(&times) { render_time(avg) } else { "DNF".to_string() },
        if let Some(std) = math::std(&times) { render_time(std) } else { "-1".to_string() },
    );
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Blue));

    frame.render_widget(paragraph, area);
}

fn render_time(time: Duration) -> String {
    format!("{:02}:{:02}:{02}", time.as_secs() / 60, time.as_secs() % 60, time.subsec_millis())
}
