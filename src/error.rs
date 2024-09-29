use crate::solve::SolveError;

use std::io;

#[derive(Debug)]
pub enum CubeError {
    Io(io::Error),
    InvalidSolve(SolveError),
    Serde(serde_json::Error)
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

impl From<serde_json::Error> for CubeError {
    fn from(err: serde_json::Error) -> Self {
        CubeError::Serde(err)
    }
}
