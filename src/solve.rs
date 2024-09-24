use std::{error::Error, fmt, time::Duration};

#[derive(Debug, Clone)]
pub enum Penalty {
    Plus2,
    DNF
}

// [TODO]: add messages to errors (better error handling)
#[derive(Debug)]
pub enum SolveError {
    // #[error("Time must be None when penalty is DNF")]
    TimeWithDNF,
    // #[error("Time cannot be None unless penalty is DNF")]
    NoTimeWithoutDNF
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SolveError::TimeWithDNF => write!(f, "Time must be None when penalty is DNF"),
            SolveError::NoTimeWithoutDNF => write!(f, "Time cannot be None unless penalty is DNF")
        }
    }
}

impl Error for SolveError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn description(&self) -> &str {
        match *self {
            Self::TimeWithDNF => "solve time but is DNF",
            Self::NoTimeWithoutDNF => "missing solve time and is not DNF"
        }
    }
}

#[derive(Debug)]
pub struct Solve {
    scramble: String,
    time: Option<Duration>,
    penalty: Option<Penalty>
}

impl Solve {
    pub fn build(scramble: String, time: Option<Duration>, penalty: Option<Penalty>) -> Result<Solve, SolveError> {
        match penalty {
            Some(Penalty::DNF) => {
                if time.is_some() {
                    return Err(SolveError::TimeWithDNF)
                }
            },
            _ => {
                if time.is_none() {
                    return Err(SolveError::NoTimeWithoutDNF)
                }
            }
        }

        Ok(Solve { scramble, time, penalty })
    }

    pub fn scramble(&self) -> &str {
        &self.scramble
    }

    pub fn time(&self) -> &Option<Duration> {
        &self.time
    }

    pub fn penalty(&self) -> &Option<Penalty> {
        &self.penalty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_solve() {
        let solve = Solve::build(
            "R U R' U'".to_owned(),
            Some(Duration::from_secs(10)),
            None
        );
        assert!(solve.is_ok());
    }

    #[test]
    fn valid_solve_with_penalty() {
        let solve = Solve::build(
            "R U R' U'".to_owned(),
            Some(Duration::from_secs(10)),
            Some(Penalty::Plus2)
        );
        assert!(solve.is_ok());
    }

    #[test]
    fn valid_solve_dnf() {
        let solve = Solve::build("R U R' U'".to_string(), None, Some(Penalty::DNF));
        assert!(solve.is_ok());
    }

    #[test]
    fn invalid_solve_time_with_dnf() {
        let solve = Solve::build(
            "R U R' U".to_owned(),
            Some(Duration::from_secs(10)),
            Some(Penalty::DNF)
        );
        assert!(matches!(solve, Err(SolveError::TimeWithDNF)));
    }

    #[test]
    fn invalid_solve_no_time_with_plus2() {
        let solve = Solve::build(
            "R U R' U".to_owned(),
            None,
            Some(Penalty::Plus2)
        );
        assert!(matches!(solve, Err(SolveError::NoTimeWithoutDNF)));
    }
}
