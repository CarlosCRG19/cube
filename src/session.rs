use crate::solve::Solve;

pub struct Session {
    solves: Vec<Solve>,
}

impl Session {
    pub fn new() -> Self {
        Session { solves: Vec::new() }
    }

    pub fn from_solves(solves: Vec<Solve>) -> Self {
        Session { solves }
    }

    pub fn solves(&self) -> &[Solve] {
        &self.solves
    }

    pub fn save_solve(&mut self, solve: Solve) {
        self.solves.push(solve);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scramble::{Puzzle, Scrambler}, solve::Penalty};
    use std::time::Duration;

    #[test]
    fn new_session() {
        let session = Session::new();
        assert_eq!(session.solves().len(), 0);
    }

    #[test]
    fn new_session_from_solves() {
        let scramble = Scrambler::new_scramble(Puzzle::Cube3x3);
        let session = Session::from_solves(vec![
            Solve::build(scramble.clone(), Some(Duration::from_millis(5440)), None).unwrap(),
            Solve::build(scramble.clone(), Some(Duration::from_millis(7480)), None).unwrap(),
            Solve::build(scramble.clone(), Some(Duration::from_millis(5000)), None).unwrap()
        ]);

        assert_eq!(session.solves().len(), 3);
    }

    #[test]
    fn save_solve() {
        let mut session = Session::new();
        let scramble = Scrambler::new_scramble(Puzzle::Cube3x3);

        session.save_solve(Solve::build(scramble.clone(), Some(Duration::from_millis(5440)), None).unwrap());
        session.save_solve(Solve::build(scramble.clone(), Some(Duration::from_millis(7480)), None).unwrap());
        session.save_solve(Solve::build(scramble.clone(),None, Some(Penalty::DNF)).unwrap());

        assert_eq!(session.solves().len(), 3);
    }
}
