//! Error type for the library

use std::error::Error as StdError;
use std::fmt::Display;

/// Error type for the library
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    /// Rule specifies more neighbours than the dimensionality of the grid allows - (neighbours, max_neighbours)
    TooHighRule(usize, usize),
    /// Life in a zero-dimensional space is not possible
    ZeroDimension,
    /// A rule with zero neighbours for birth is invalid (infinite number of cells would be born)
    ZeroNeighbourBirthRule,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooHighRule(neighbours, max_neighbours) => write!(
                f,
                "A rule specifies more neighbours ({}) than the dimensionality of the grid allows (max {})",
                neighbours, max_neighbours
            ),
            Self::ZeroDimension => write!(f, "Life in a zero-dimensional space is not possible"),
            Self::ZeroNeighbourBirthRule => write!(f, "A rule with zero neighbours for birth is invalid (infinite number of cells would be born)"),
        }
    }
}
impl StdError for Error {}
