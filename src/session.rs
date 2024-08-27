use std::time::Duration;
pub struct Session {
    times: Vec<Duration>,
}

impl Session {
    pub fn new() -> Self {
        Session { times: Vec::new() }
    }

    pub fn from_times(times: Vec<Duration>) -> Self {
        Session { times }
    }

    pub fn times(&self) -> &Vec<Duration> {
        &self.times
    }

    pub fn save_time(&mut self, time: Duration) {
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