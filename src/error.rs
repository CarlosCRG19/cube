use crate::solve::SolveError;

use std::io;

#[derive(Debug)]
pub enum CubeError {
    Io(io::Error),
    InvalidSolve(SolveError)
}

impl From<SolveError> for CubeError {
    fn from(err: SolveError) -> Self {
        CubeError::InvalidSolve(err)
    }
}

impl From<io::Error> for CubeError {
    fn from(err: io::Error) -> Self {
        CubeError::Io(err)
    }
}
