use crate::scramble::{Puzzle, Scrambler};
use crate::session::Session;
use crate::solve::Solve;
use crate::timer::{Timer, TimerState};

use ratatui::crossterm::event::KeyCode;

pub struct App {
    pub should_quit: bool,
    pub timer: Timer,
    pub session: Session,
    pub current_scramble: Option<String>, 
}

impl App {
    pub fn new() -> App {
        App {
            timer: Timer::new(),
            should_quit: false,
            session: Session::new(),
            current_scramble: Some(Scrambler::new_scramble(Puzzle::Cube3x3))
        }
    }

    pub fn on_key_pressed(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(' ') => {
                match self.timer.state() {
                    TimerState::Running { .. } => {
                        let current_scramble = self.current_scramble.take().unwrap();
                        let solve = Solve::build(current_scramble, Some(self.timer.elapsed()), None).unwrap();
                        self.timer.pause();
                        self.session.save_solve(solve);
                        self.current_scramble = Some(Scrambler::new_scramble(Puzzle::Cube3x3));
                    }
                    _ => {
                        self.timer.reset();
                        self.timer.start();
                    }
                }
            },
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }
}
