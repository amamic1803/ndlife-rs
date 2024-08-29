//! ***ndlife*** is an implementation of infinite, N-dimensional game of life in Rust.
//!
//! A game of life is a cellular automaton devised by the British mathematician John Horton Conway in 1970.
//! The game is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input.
//! One interacts with the Game of Life by creating an initial configuration and observing how it evolves.
//! This crate extends the game of life to N dimensions, where N is any positive integer.
//!
//! # Example
//! ```
//! use std::collections::HashSet;
//! use ndlife::Life;
//!
//! // setup conway's game of life
//!
//! let mut birth_rules = HashSet::with_capacity(1);
//! birth_rules.insert(3);
//!
//! let mut survival_rules = HashSet::with_capacity(2);
//! survival_rules.insert(2);
//! survival_rules.insert(3);
//!
//! let mut life = Life::<2>::new(birth_rules, survival_rules).unwrap();
//!
//! // or use shortcut
//! // let mut life = conways_game_of_life();
//!
//! // glider pattern
//! let mut alive_cells = HashSet::with_capacity(5);
//! alive_cells.insert([0, 0]);
//! alive_cells.insert([1, 0]);
//! alive_cells.insert([2, 0]);
//! alive_cells.insert([2, 1]);
//! alive_cells.insert([1, 2]);
//!
//! // set initial state
//! life.set_alive_cells(alive_cells);
//!
//! // advance life by 4 generations (repeat cycle for glider)
//! for _ in 0..4 {
//!    life.next_generation();
//! }
//!
//! // glider moves one cell diagonally (right-down) every 4 generations
//! let mut expected_alive_cells = HashSet::with_capacity(5);
//! expected_alive_cells.insert([1, -1]);
//! expected_alive_cells.insert([2, -1]);
//! expected_alive_cells.insert([3, -1]);
//! expected_alive_cells.insert([3, 0]);
//! expected_alive_cells.insert([2, 1]);
//!
//! // assert that is indeed what happened
//! assert_eq!(life.alive_cells(), &expected_alive_cells);
//! ```

pub mod error;
pub mod life;

#[doc(inline)]
pub use life::*;

#[doc(inline)]
pub use error::*;
