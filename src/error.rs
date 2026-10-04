//! An error type for the library.

use std::error::Error as StdError;
use std::fmt::Display;

/// An error type for the library.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    /// The rule specifies more neighbors than the dimensionality allows (`neighbors` > [MAX_NEIGHBORS](crate::Life::MAX_NEIGHBORS)).
    TooHighRule(usize, usize),
    /// A life in a zero-dimensional space is not possible.
    ZeroDimension,
    /// A birth rule with zero neighbors is invalid (infinite number of cells would be born).
    ZeroNeighbourBirthRule,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooHighRule(neighbours, max_neighbours) => write!(
                f,
                "The rule specifies more neighbours ({}) than the dimensionality allows (max {})",
                neighbours, max_neighbours
            ),
            Self::ZeroDimension => write!(f, "A life in a zero-dimensional space is not possible."),
            Self::ZeroNeighbourBirthRule => write!(
                f,
                "A birth rule with zero neighbors is invalid (infinite number of cells would be born)."
            ),
        }
    }
}
impl StdError for Error {}
