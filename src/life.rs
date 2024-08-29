//! Implementation of infinite N-dimensional game of life

use crate::error::Error;
use std::collections::{HashMap, HashSet};

/// Infinite N-dimensional game of life
/// # Example
/// ```
/// use ndlife::life::Life;
/// use std::collections::HashSet;
///
/// let birth_rules: HashSet<usize> = [3].into_iter().collect();
/// let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
/// let alive_cells: HashSet<[i64; 2]> = [[0, 0], [1, 0], [2, 0], [2, 1], [1, 2]].into_iter().collect();
///
/// let mut life = Life::new_with_alive_cells(birth_rules, survival_rules, alive_cells).unwrap();
///
/// // advance 12 generations
/// for _ in 0..12 {
///    life.next_generation();
/// }
///
/// assert_eq!(life.age(), 12);
///
/// let expected_alive_cells: HashSet<[i64; 2]> = [[3, -3], [4, -3], [5, -3], [5, -2], [4, -1]].into_iter().collect();
/// assert_eq!(life.alive_cells(), &expected_alive_cells);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Life<const N: usize> {
    /// The age of the life.
    age: u64,
    /// The rules for a dead cell to become alive.
    birth_rules: HashSet<usize>,
    /// The rules for alive cell to stay alive.
    survival_rules: HashSet<usize>,
    /// The alive cells.
    alive_cells: HashSet<[i64; N]>,
    /// The alive cells in the previous generation.
    prev_alive: HashSet<[i64; N]>,
    /// The number of alive neighbours for each dead cell, used in the [next_generation] method.
    dead_neighbours: HashMap<[i64; N], usize>,
}
impl<const N: usize> Life<N> {
    /// Maximum number of neighbours a cell can have with given dimension `N`.
    pub const MAX_NEIGHBOURS: usize = const { 3usize.pow(N as u32) - 1 };

    /// Create a new game of life with given birth and survival rules.
    /// # Arguments
    /// * `birth_rules` - A set of number of neighbours required for a dead cell to become alive.
    /// * `survival_rules` - A set of number of neighbours required for a live cell to stay alive.
    /// # Returns
    /// A [Result] containing a new game of life if successful, or an error.
    /// # Errors
    /// * [TooHighRule](Error::TooHighRule) - If any rule is greater than [MAX_NEIGHBOURS](Self::MAX_NEIGHBOURS).
    /// * [ZeroDimension](Error::ZeroDimension) - If `N` is 0.
    /// * [ZeroNeighbourBirthRule](Error::ZeroNeighbourBirthRule) - If birth_rules contains 0.
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use std::collections::HashSet;
    ///
    /// let birth_rules: HashSet<usize> = [3].into_iter().collect();
    /// let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
    ///
    /// let life = Life::<2>::new(birth_rules.clone(), survival_rules.clone()).unwrap();
    ///
    /// assert_eq!(life.birth_rules(), &birth_rules);
    /// assert_eq!(life.survival_rules(), &survival_rules);
    /// assert_eq!(life.alive_cells(), &HashSet::new());
    /// ```
    pub fn new(birth_rules: HashSet<usize>, survival_rules: HashSet<usize>) -> Result<Self, Error> {
        Self::new_with_alive_cells(birth_rules, survival_rules, HashSet::new())
    }

    /// Create a new game of life with given birth and survival rules and alive cells.
    /// # Arguments
    /// * `birth_rules` - A set of number of neighbours required for a dead cell to become alive.
    /// * `survival_rules` - A set of number of neighbours required for a live cell to stay alive.
    /// * `alive_cells` - A set of coordinates of alive cells.
    /// # Returns
    /// A [Result] containing a new game of life if successful, or an error.
    /// # Errors
    /// * [TooHighRule](Error::TooHighRule) - If any rule is greater than [MAX_NEIGHBOURS](Self::MAX_NEIGHBOURS).
    /// * [ZeroDimension](Error::ZeroDimension) - If `N` is 0.
    /// * [ZeroNeighbourBirthRule](Error::ZeroNeighbourBirthRule) - If birth_rules contains 0.
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use std::collections::HashSet;
    ///
    /// let birth_rules: HashSet<usize> = [3].into_iter().collect();
    /// let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
    /// let mut alive_cells = HashSet::new();
    /// alive_cells.insert([0, 0]);
    ///
    /// let life = Life::new_with_alive_cells(birth_rules.clone(), survival_rules.clone(), alive_cells.clone()).unwrap();
    ///
    /// assert_eq!(life.birth_rules(), &birth_rules);
    /// assert_eq!(life.survival_rules(), &survival_rules);
    /// assert_eq!(life.alive_cells(), &alive_cells);
    /// ```
    pub fn new_with_alive_cells(birth_rules: HashSet<usize>, survival_rules: HashSet<usize>, alive_cells: HashSet<[i64; N]>) -> Result<Self, Error> {
        if N == 0 {
            return Err(Error::ZeroDimension);
        }
        if birth_rules.contains(&0) {
            return Err(Error::ZeroNeighbourBirthRule);
        }
        for rule in birth_rules.iter().chain(survival_rules.iter()) {
            if *rule > Self::MAX_NEIGHBOURS {
                return Err(Error::TooHighRule(*rule, Self::MAX_NEIGHBOURS));
            }
        }
        Ok(Self {
            age: 0,
            birth_rules,
            survival_rules,
            alive_cells,
            prev_alive: HashSet::new(),
            dead_neighbours: HashMap::new(),
        })
    }

    /// Get the age of the game of life.
    pub fn age(&self) -> u64 {
        self.age
    }

    /// Get the birth rules of the game of life.
    pub fn birth_rules(&self) -> &HashSet<usize> {
        &self.birth_rules
    }

    /// Set the birth rules for the game of life.
    /// # Arguments
    /// * `birth_rules` - A set of number of neighbours required for a dead cell to become alive.
    /// # Returns
    /// A [Result] containing `()` if successful, or an error.
    /// # Errors
    /// * [TooHighRule](Error::TooHighRule) - If any rule is greater than [MAX_NEIGHBOURS](Self::MAX_NEIGHBOURS).
    /// * [ZeroNeighbourBirthRule](Error::ZeroNeighbourBirthRule) - If birth_rules contains 0.
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use ndlife::error::Error;
    /// use std::collections::HashSet;
    ///
    /// let mut life = Life::<2>::new(HashSet::new(), HashSet::new()).unwrap();
    ///
    /// let birth_rules: HashSet<usize> = [3].into_iter().collect();
    /// life.set_birth_rules(birth_rules.clone()).unwrap();
    /// assert_eq!(life.birth_rules(), &birth_rules);
    ///
    /// let birth_rules: HashSet<usize> = [0].into_iter().collect();
    /// assert_eq!(life.set_birth_rules(birth_rules), Err(Error::ZeroNeighbourBirthRule));
    ///
    /// let birth_rules: HashSet<usize> = [9].into_iter().collect();
    /// assert_eq!(life.set_birth_rules(birth_rules), Err(Error::TooHighRule(9, 8)));
    /// ```
    pub fn set_birth_rules(&mut self, birth_rules: HashSet<usize>) -> Result<(), Error> {
        if birth_rules.contains(&0) {
            return Err(Error::ZeroNeighbourBirthRule);
        }
        for rule in birth_rules.iter() {
            if *rule > Self::MAX_NEIGHBOURS {
                return Err(Error::TooHighRule(*rule, Self::MAX_NEIGHBOURS));
            }
        }
        self.birth_rules = birth_rules;
        Ok(())
    }

    /// Get the survival rules of the game of life.
    pub fn survival_rules(&self) -> &HashSet<usize> {
        &self.survival_rules
    }

    /// Set the survival rules for the game of life.
    /// # Arguments
    /// * `survival_rules` - A set of number of neighbours required for a live cell to stay alive.
    /// # Returns
    /// A [Result] containing `()` if successful, or an error.
    /// # Errors
    /// * [TooHighRule](Error::TooHighRule) - If any rule is greater than [MAX_NEIGHBOURS](Self::MAX_NEIGHBOURS).
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use ndlife::error::Error;
    /// use std::collections::HashSet;
    ///
    /// let mut life = Life::<2>::new(HashSet::new(), HashSet::new()).unwrap();
    ///
    /// let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
    /// life.set_survival_rules(survival_rules.clone()).unwrap();
    /// assert_eq!(life.survival_rules(), &survival_rules);
    ///
    /// let survival_rules: HashSet<usize> = [9].into_iter().collect();
    /// assert_eq!(life.set_survival_rules(survival_rules), Err(Error::TooHighRule(9, 8)));
    /// ```
    pub fn set_survival_rules(&mut self, survival_rules: HashSet<usize>) -> Result<(), Error> {
        for rule in survival_rules.iter() {
            if *rule > Self::MAX_NEIGHBOURS {
                return Err(Error::TooHighRule(*rule, Self::MAX_NEIGHBOURS));
            }
        }
        self.survival_rules = survival_rules;
        Ok(())
    }

    /// Get the alive cells in the game of life.
    pub fn alive_cells(&self) -> &HashSet<[i64; N]> {
        &self.alive_cells
    }

    /// Set the alive cells for the game of life.
    /// # Arguments
    /// * `alive_cells` - A set of coordinates of alive cells.
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use std::collections::HashSet;
    ///
    /// let mut life = Life::<2>::new(HashSet::new(), HashSet::new()).unwrap();
    ///
    /// let mut alive_cells = HashSet::new();
    /// alive_cells.insert([0, 0]);
    /// life.set_alive_cells(alive_cells.clone());
    /// assert_eq!(life.alive_cells(), &alive_cells);
    /// ```
    pub fn set_alive_cells(&mut self, alive_cells: HashSet<[i64; N]>) {
        self.alive_cells = alive_cells;
    }

    /// Get whether a cell is alive.
    /// # Arguments
    /// * `cell` - Coordinates of the cell.
    /// # Returns
    /// * [bool] - Whether the cell is alive.
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use std::collections::HashSet;
    ///
    /// let alive_cells: HashSet<[i64; 2]> = [[1, 1]].into_iter().collect();
    /// let life = Life::new_with_alive_cells(HashSet::new(), HashSet::new(), alive_cells).unwrap();
    ///
    /// assert_eq!(life.get_cell(&[1, 1]), true);
    /// assert_eq!(life.get_cell(&[0, 0]), false);
    /// ```
    pub fn get_cell(&self, cell: &[i64; N]) -> bool {
        self.alive_cells.contains(cell)
    }

    /// Set a cell as alive or dead.
    /// # Arguments
    /// * `cell` - Coordinates of the cell.
    /// * `state` - Whether the cell should be alive.
    /// # Returns
    /// * [bool] - Whether the cell was changed.
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use std::collections::HashSet;
    ///
    /// let mut alive_cells = HashSet::new();
    /// alive_cells.insert([1, 1]);
    /// let mut life = Life::new_with_alive_cells(HashSet::new(), HashSet::new(), alive_cells).unwrap();
    ///
    /// assert_eq!(life.set_cell(&[1, 1], false), true);
    /// assert_eq!(life.set_cell(&[0, 0], true), true);
    /// assert_eq!(life.set_cell(&[0, 0], true), false);
    /// ```
    pub fn set_cell(&mut self, cell: &[i64; N], state: bool) -> bool {
        if state {
            self.alive_cells.insert(*cell)
        } else {
            self.alive_cells.remove(cell)
        }
    }

    /// Toggle a cell between alive and dead.
    /// # Arguments
    /// * `cell` - Coordinates of the cell.
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use std::collections::HashSet;
    ///
    /// let mut alive_cells = HashSet::new();
    /// alive_cells.insert([1, 1]);
    /// let mut life = Life::new_with_alive_cells(HashSet::new(), HashSet::new(), alive_cells).unwrap();
    ///
    /// life.toggle_cell(&[1, 1]);
    /// life.toggle_cell(&[0, 0]);
    ///
    /// let expected_alive_cells: HashSet<[i64; 2]> = [[0, 0]].into_iter().collect();
    /// assert_eq!(life.alive_cells(), &expected_alive_cells);
    /// ```
    pub fn toggle_cell(&mut self, cell: &[i64; N]) {
        if !self.alive_cells.remove(cell) {
            self.alive_cells.insert(*cell);
        }
    }

    /// Advance the game of life to the next generation.
    pub fn next_generation(&mut self) {
        let deltas = || {
            let mut ptr = 0;
            let mut deltas = [-1i64; N];
            deltas[ptr] = -2;
            std::iter::from_fn(move || {
                while ptr < N {
                    if deltas[ptr] == 1 {
                        ptr += 1;
                    } else {
                        deltas[ptr] += 1;
                        deltas[0..ptr].fill(-1);
                        ptr = 0;
                        return Some(deltas);
                    }
                }
                None
            })
            .filter(|deltas| deltas.iter().any(|&delta| delta != 0))
        };

        self.age += 1;
        std::mem::swap(&mut self.alive_cells, &mut self.prev_alive);
        self.alive_cells.clear();
        self.dead_neighbours.clear();

        self.prev_alive.iter().for_each(|alive_cell| {
            let mut alive_neighbours = 0;
            for delta in deltas() {
                let neighbour = std::array::from_fn(|i| alive_cell[i] + delta[i]);
                if self.prev_alive.contains(&neighbour) {
                    alive_neighbours += 1;
                } else {
                    *self.dead_neighbours.entry(neighbour).or_insert(0) += 1;
                }
            }
            if self.survival_rules.contains(&alive_neighbours) {
                self.alive_cells.insert(*alive_cell);
            }
        });

        for (key, value) in self.dead_neighbours.iter() {
            if self.birth_rules.contains(value) {
                self.alive_cells.insert(*key);
            }
        }
    }

    /// Get the cells that have changed between the previous and current generation.
    /// # Returns
    /// An iterator over the coordinates of changed cells.
    /// # Example
    /// ```
    /// use ndlife::life::Life;
    /// use std::collections::HashSet;
    ///
    /// let mut alive_cells = HashSet::new();
    /// alive_cells.insert([1, 1]);
    /// let mut life = Life::new_with_alive_cells(HashSet::new(), HashSet::new(), alive_cells).unwrap();
    ///
    /// life.next_generation();
    /// assert_eq!(vec![[1, 1]], life.changed_cells().copied().collect::<Vec<_>>());
    /// ```
    pub fn changed_cells(&self) -> impl Iterator<Item = &[i64; N]> {
        self.prev_alive.symmetric_difference(&self.alive_cells)
    }
}

/// Create new game of life with Conway's rules
///
/// The life is 2-dimensional and the birth rules are [3] and the survival rules are [2, 3].
/// # Example
/// ```
/// use ndlife::life::{conways_game_of_life, Life};
///
/// let conways_life = conways_game_of_life();
/// let conways_life_manual = Life::<2>::new([3].into_iter().collect(), [2, 3].into_iter().collect()).unwrap();
///
/// assert_eq!(conways_life, conways_life_manual);
/// ```
pub fn conways_game_of_life() -> Life<2> {
    Life::new([3].into_iter().collect(), [2, 3].into_iter().collect()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_neighbours() {
        assert_eq!(Life::<1>::MAX_NEIGHBOURS, 2);
        assert_eq!(Life::<2>::MAX_NEIGHBOURS, 8);
        assert_eq!(Life::<3>::MAX_NEIGHBOURS, 26);
    }

    #[test]
    fn test_new() {
        let birth_rules: HashSet<usize> = [3].into_iter().collect();
        let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
        let life = Life::<2>::new(birth_rules.clone(), survival_rules.clone()).unwrap();
        assert_eq!(life.birth_rules(), &birth_rules);
        assert_eq!(life.survival_rules(), &survival_rules);
        assert_eq!(life.alive_cells(), &HashSet::new());
    }

    #[test]
    fn test_new_with_alive_cells() {
        let birth_rules: HashSet<usize> = [3].into_iter().collect();
        let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
        let mut alive_cells = HashSet::new();
        alive_cells.insert([0, 0]);
        let life = Life::new_with_alive_cells(birth_rules.clone(), survival_rules.clone(), alive_cells.clone()).unwrap();
        assert_eq!(life.birth_rules(), &birth_rules);
        assert_eq!(life.survival_rules(), &survival_rules);
        assert_eq!(life.alive_cells(), &alive_cells);
    }

    #[test]
    fn test_age() {
        let mut life = Life::<2>::new(HashSet::new(), HashSet::new()).unwrap();
        (0..100).for_each(|_| life.next_generation());
        assert_eq!(life.age(), 100);
    }

    #[test]
    fn test_birth_rules() {
        let mut life = Life::<2>::new(HashSet::new(), HashSet::new()).unwrap();
        let birth_rules: HashSet<usize> = [3].into_iter().collect();
        life.set_birth_rules(birth_rules.clone()).unwrap();
        assert_eq!(life.birth_rules(), &birth_rules);
        let birth_rules: HashSet<usize> = [0].into_iter().collect();
        assert_eq!(life.set_birth_rules(birth_rules), Err(Error::ZeroNeighbourBirthRule));
        let birth_rules: HashSet<usize> = [9].into_iter().collect();
        assert_eq!(life.set_birth_rules(birth_rules), Err(Error::TooHighRule(9, 8)));
    }

    #[test]
    fn test_survival_rules() {
        let mut life = Life::<2>::new(HashSet::new(), HashSet::new()).unwrap();
        let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
        life.set_survival_rules(survival_rules.clone()).unwrap();
        assert_eq!(life.survival_rules(), &survival_rules);
        let survival_rules: HashSet<usize> = [9].into_iter().collect();
        assert_eq!(life.set_survival_rules(survival_rules), Err(Error::TooHighRule(9, 8)));
    }

    #[test]
    fn test_alive_cells() {
        let mut life = Life::<2>::new(HashSet::new(), HashSet::new()).unwrap();
        assert_eq!(life.alive_cells(), &HashSet::new());

        let mut alive_cells = HashSet::new();
        alive_cells.insert([0, 0]);
        life.set_alive_cells(alive_cells.clone());
        assert_eq!(life.alive_cells(), &alive_cells);
    }

    #[test]
    fn test_get_cell() {
        let alive_cells: HashSet<[i64; 2]> = [[1, 1]].into_iter().collect();
        let life = Life::new_with_alive_cells(HashSet::new(), HashSet::new(), alive_cells).unwrap();
        assert!(life.get_cell(&[1, 1]));
        assert!(!life.get_cell(&[0, 0]));
    }

    #[test]
    fn test_set_cell() {
        let mut alive_cells = HashSet::new();
        alive_cells.insert([1, 1]);
        let mut life = Life::new_with_alive_cells(HashSet::new(), HashSet::new(), alive_cells).unwrap();
        assert!(life.set_cell(&[1, 1], false));
        assert!(life.set_cell(&[0, 0], true));
        assert!(!life.set_cell(&[0, 0], true));
    }

    #[test]
    fn test_toggle_cell() {
        let mut alive_cells = HashSet::new();
        alive_cells.insert([1, 1]);
        let mut life = Life::new_with_alive_cells(HashSet::new(), HashSet::new(), alive_cells).unwrap();
        life.toggle_cell(&[1, 1]);
        life.toggle_cell(&[0, 0]);
        let expected_alive_cells: HashSet<[i64; 2]> = [[0, 0]].into_iter().collect();
        assert_eq!(life.alive_cells(), &expected_alive_cells);
    }

    #[test]
    fn test_next_generation() {
        let birth_rules: HashSet<usize> = [3].into_iter().collect();
        let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
        let alive_cells: HashSet<[i64; 2]> = [[0, 0], [1, 0], [2, 0], [2, 1], [1, 2]].into_iter().collect();
        let mut life = Life::new_with_alive_cells(birth_rules, survival_rules, alive_cells).unwrap();
        for _ in 0..12 {
            life.next_generation();
        }
        assert_eq!(life.age(), 12);
        let expected_alive_cells: HashSet<[i64; 2]> = [[3, -3], [4, -3], [5, -3], [5, -2], [4, -1]].into_iter().collect();
        assert_eq!(life.alive_cells(), &expected_alive_cells);
    }

    #[test]
    fn test_changed_cells() {
        let mut alive_cells = HashSet::new();
        alive_cells.insert([1, 1]);
        let mut life = Life::new_with_alive_cells(HashSet::new(), HashSet::new(), alive_cells).unwrap();
        life.next_generation();
        assert_eq!(vec![[1, 1]], life.changed_cells().copied().collect::<Vec<_>>());
    }
}
