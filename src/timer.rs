use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum TimerState {
    Idle,
    Paused { elapsed: Duration },
    Running { start: Instant }
}

#[derive(Debug)]
pub struct Timer {
    state: TimerState
}

impl Timer {
    pub fn new() -> Self {
        Timer { state: TimerState::Idle }
    }

    pub fn state(&self) -> &TimerState {
        &self.state
    }

    pub fn start(&mut self) {
        self.state = match self.state {
            TimerState::Idle => TimerState::Running { start: Instant::now() },
            TimerState::Paused { elapsed } => {
                TimerState::Running { start: Instant::now() - elapsed }
            },
            TimerState::Running { .. }=> return,
        }
    }

    pub fn pause(&mut self) {
        if let TimerState::Running { start } = self.state {
            self.state = TimerState::Paused { elapsed: start.elapsed() }
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Idle;
    }

    pub fn elapsed(&self) -> Duration {
        match self.state {
            TimerState::Idle => Duration::ZERO,
            TimerState::Running { start } => start.elapsed(),
            TimerState::Paused { elapsed } => elapsed,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    const SLEEP_TIME: Duration = Duration::from_millis(10);

    fn sleep_some_time() {
        thread::sleep(SLEEP_TIME);
    }

    #[test]
    fn new_timer() {
        let timer = Timer::new();
        assert!(matches!(timer.state, TimerState::Idle));
    }

    #[test]
    fn start_from_idle() {
        let mut timer = Timer::new();
        timer.start();
        assert!(matches!(timer.state, TimerState::Running { .. }))
    }

    #[test]
    fn start_from_paused() {
        let mut timer = Timer::new();
        timer.start();
        sleep_some_time();

        timer.pause();
        let paused_time = timer.elapsed();

        timer.start();
        sleep_some_time();

        assert!(matches!(timer.state, TimerState::Running { .. }));
        assert!(timer.elapsed() > paused_time);
    }

    #[test]
    fn elapsed() {
        let mut timer = Timer::new();
        timer.start();
        sleep_some_time();

        let elapsed = timer.elapsed();
        assert!(elapsed >= SLEEP_TIME);
        assert!(elapsed < SLEEP_TIME * 2);
    }

    #[test]
    fn pause() {
        let mut timer = Timer::new();
        timer.start();
        sleep_some_time();

        timer.pause();
        let paused_time = timer.elapsed();

        sleep_some_time();
        assert_eq!(timer.elapsed(), paused_time);
    }

    #[test]
    fn reset() {
        let mut timer = Timer::new();
        timer.start();
        sleep_some_time();

        timer.reset();
        assert!(matches!(timer.state, TimerState::Idle));
        assert_eq!(timer.elapsed(), Duration::ZERO);
    }

    #[test]
    fn multiple_start_reset_cycles() {
        let mut timer = Timer::new();

        timer.start();
        sleep_some_time();
        timer.reset();

        timer.start();
        sleep_some_time();
        timer.pause();

        let paused_time = timer.elapsed();
        assert!(paused_time >= SLEEP_TIME);

        timer.start();
        sleep_some_time();

        assert!(timer.elapsed() > paused_time);
    }
}
