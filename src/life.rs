//! An implementation of infinite, N-dimensional game of life.

use crate::error::Error;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

/// An infinite, N-dimensional game of life.
/// # Example
/// ```
/// use ndlife::Life;
/// use std::collections::HashSet;
///
/// let birth_rules = [3];
/// let survival_rules = [2, 3];
/// let mut life = Life::new(birth_rules, survival_rules).unwrap();
/// life.alive_cells_mut().extend([[0, 0], [1, 0], [2, 0], [2, 1], [1, 2]]);
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
    /// The age (number of simulation steps) of the life simulation.
    age: i64,
    /// The rules for a dead cell to become alive.
    birth_rules: HashSet<usize>,
    /// The rules for an alive cell to stay alive.
    survival_rules: HashSet<usize>,
    /// The set of alive cells.
    alive_cells: HashSet<[i64; N]>,
    /// The set of alive cells in the previous generation.
    prev_alive: HashSet<[i64; N]>,
    /// The number of alive neighbors for each dead cell, used in the [next_generation] method.
    dead_neighbors: HashMap<[i64; N], usize>,
}
impl<const N: usize> Life<N> {
    /// The maximum number of neighbors (alive cells) a cell can have.
    pub const MAX_NEIGHBORS: usize = const { 3usize.pow(N as u32) - 1 };

    /// Create a new [Life] with given birth and survival rules.
    ///
    /// The birth rules define the number of alive neighbors a dead cell must have to become alive in the next generation.
    /// The survival rules define the number of alive neighbors an alive cell must have to stay alive in the next generation.
    /// # Arguments
    /// * `birth_rules` - An iterator over the birth rules of the game of life.
    /// * `survival_rules` - An iterator over the survival rules of the game of life.
    /// # Returns
    /// * A new game of life, or an error.
    /// # Errors
    /// * [TooHighRule](Error::TooHighRule) - If any rule is greater than [MAX_NEIGHBORS](Self::MAX_NEIGHBORS).
    /// * [ZeroDimension](Error::ZeroDimension) - If `N` is 0.
    /// * [ZeroNeighbourBirthRule](Error::ZeroNeighbourBirthRule) - If birth_rules contains `0`.
    /// # Example
    /// ```
    /// use ndlife::Life;
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
    pub fn new<T, U, V, W>(birth_rules: T, survival_rules: U) -> Result<Self, Error>
    where
        T: IntoIterator<Item = V>,
        U: IntoIterator<Item = W>,
        V: Borrow<usize>,
        W: Borrow<usize>,
    {
        if N == 0 {
            return Err(Error::ZeroDimension);
        }
        let mut birth_rules_set = HashSet::new();
        let mut survival_rules_set = HashSet::new();

        for rule in birth_rules.into_iter() {
            let rule = *rule.borrow();
            if rule == 0 {
                return Err(Error::ZeroNeighbourBirthRule);
            }
            if rule > Self::MAX_NEIGHBORS {
                return Err(Error::TooHighRule(rule, Self::MAX_NEIGHBORS));
            }
            birth_rules_set.insert(rule);
        }

        for rule in survival_rules.into_iter() {
            let rule = *rule.borrow();
            if rule > Self::MAX_NEIGHBORS {
                return Err(Error::TooHighRule(rule, Self::MAX_NEIGHBORS));
            }
            survival_rules_set.insert(rule);
        }

        Ok(Self {
            age: 0,
            birth_rules: birth_rules_set,
            survival_rules: survival_rules_set,
            alive_cells: HashSet::new(),
            prev_alive: HashSet::new(),
            dead_neighbors: HashMap::new(),
        })
    }

    /// Get the age of the game of life.
    ///
    /// The age is defined as the number of generations that have passed since the creation of the game of life.
    /// For newly created game of life, the age is `0`.
    /// # Returns
    /// * The age of the game of life.
    pub fn age(&self) -> i64 {
        self.age
    }

    /// Get the birth rules of the game of life.
    /// # Returns
    /// * A set of the birth rules of the game of life.
    pub fn birth_rules(&self) -> &HashSet<usize> {
        &self.birth_rules
    }

    /// Get the survival rules of the game of life.
    /// # Returns
    /// * A set of the survival rules of the game of life.
    pub fn survival_rules(&self) -> &HashSet<usize> {
        &self.survival_rules
    }

    /// Get a reference to the set of alive cells in the game of life.
    /// # Returns
    /// * A reference to the set of alive cells in the game of life.
    pub fn alive_cells(&self) -> &HashSet<[i64; N]> {
        &self.alive_cells
    }

    /// Get a mutable reference to the set of alive cells in the game of life.
    /// # Returns
    /// * A mutable reference to the set of alive cells in the game of life.
    pub fn alive_cells_mut(&mut self) -> &mut HashSet<[i64; N]> {
        &mut self.alive_cells
    }

    /// Check if a cell with given coordinates is alive.
    /// # Arguments
    /// * `cell` - The coordinates of the cell.
    /// # Returns
    /// * `true` if the cell is alive, `false` otherwise.
    /// # Example
    /// ```
    /// use ndlife::Life;
    ///
    /// let mut life = Life::default();
    /// life.alive_cells_mut().insert([1, 1]);
    ///
    /// assert!(life.get_cell(&[1, 1]));
    /// assert!(!life.get_cell(&[0, 0]));
    /// ```
    pub fn get_cell(&self, cell: &[i64; N]) -> bool {
        self.alive_cells.contains(cell)
    }

    /// Set a cell as alive or dead.
    /// # Arguments
    /// * `cell` - The coordinates of the cell.
    /// * `state` - The new state of the cell, `true` for alive, `false` for dead.
    /// # Returns
    /// * `true` if the cell state was changed, `false` if it was already in the desired state.
    /// # Example
    /// ```
    /// use ndlife::Life;
    /// use std::collections::HashSet;
    ///
    /// let mut life = Life::default();
    /// life.alive_cells_mut().insert([1, 1]);
    ///
    /// assert!(life.set_cell(&[1, 1], false));
    /// assert!(life.set_cell(&[0, 0], true));
    /// assert!(!life.set_cell(&[0, 0], true));
    /// ```
    pub fn set_cell(&mut self, cell: &[i64; N], state: bool) -> bool {
        if state {
            self.alive_cells.insert(*cell)
        } else {
            self.alive_cells.remove(cell)
        }
    }

    /// Toggle the state of a cell between alive and dead.
    /// # Arguments
    /// * `cell` - The coordinates of the cell.
    /// # Example
    /// ```
    /// use ndlife::Life;
    /// use std::collections::HashSet;
    ///
    /// let mut life = Life::default();
    /// life.alive_cells_mut().insert([1, 1]);
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

    /// Advance the game of life simulation by one generation.
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
        self.dead_neighbors.clear();

        self.prev_alive.iter().for_each(|alive_cell| {
            let mut alive_neighbours = 0;
            for delta in deltas() {
                let neighbour = std::array::from_fn(|i| alive_cell[i] + delta[i]);
                if self.prev_alive.contains(&neighbour) {
                    alive_neighbours += 1;
                } else {
                    *self.dead_neighbors.entry(neighbour).or_insert(0) += 1;
                }
            }
            if self.survival_rules.contains(&alive_neighbours) {
                self.alive_cells.insert(*alive_cell);
            }
        });

        for (key, value) in self.dead_neighbors.iter() {
            if self.birth_rules.contains(value) {
                self.alive_cells.insert(*key);
            }
        }
    }

    /// Get the cells that have changed between the previous and current generation.
    /// # Returns
    /// * An iterator over the coordinates of changed cells.
    /// # Example
    /// ```
    /// use ndlife::Life;
    /// use std::collections::HashSet;
    ///
    /// let mut life = Life::default();
    /// life.alive_cells_mut().insert([1, 1]);
    ///
    /// life.next_generation();
    /// assert_eq!(vec![[1, 1]], life.changed_cells().copied().collect::<Vec<_>>());
    /// ```
    pub fn changed_cells(&self) -> impl Iterator<Item = &[i64; N]> {
        self.prev_alive.symmetric_difference(&self.alive_cells)
    }
}
impl<const N: usize> Default for Life<N> {
    fn default() -> Self {
        Self::new([3], [2, 3]).unwrap()
    }
}

/// Create a new 2-dimensional [Life] with Conway's rules.
///
/// The created [Life] is 2-dimensional with birth rules: `[3]` and survival rules: `[2, 3]`.
/// # Example
/// ```
/// use ndlife::{conways_game_of_life, Life};
///
/// let conways_life = conways_game_of_life();
/// let conways_life_manual = Life::<2>::new([3], [2, 3]).unwrap();
///
/// assert_eq!(conways_life, conways_life_manual);
/// ```
pub fn conways_game_of_life() -> Life<2> {
    Life::<2>::default() // default creates a Life with Conway's rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_neighbors() {
        assert_eq!(Life::<1>::MAX_NEIGHBORS, 2);
        assert_eq!(Life::<2>::MAX_NEIGHBORS, 8);
        assert_eq!(Life::<3>::MAX_NEIGHBORS, 26);
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
    fn test_age() {
        let mut life = Life::<2>::default();
        (0..100).for_each(|_| life.next_generation());
        assert_eq!(life.age(), 100);
    }

    #[test]
    fn test_birth_rules() {
        let birth_rules: HashSet<usize> = [3].into_iter().collect();
        let life = Life::<2>::new(birth_rules.clone(), HashSet::<usize>::new()).unwrap();
        assert_eq!(life.birth_rules(), &birth_rules);

        let birth_rules = [0];
        let life = Life::<2>::new(birth_rules, HashSet::<usize>::new());
        assert_eq!(life, Err(Error::ZeroNeighbourBirthRule));

        let birth_rules = [9];
        let life = Life::<2>::new(birth_rules, HashSet::<usize>::new());
        assert_eq!(life, Err(Error::TooHighRule(9, 8)));
    }

    #[test]
    fn test_survival_rules() {
        let survival_rules: HashSet<usize> = [2, 3].into_iter().collect();
        let life = Life::<2>::new(HashSet::<usize>::new(), survival_rules.clone()).unwrap();
        assert_eq!(life.survival_rules(), &survival_rules);
        let survival_rules = [9];
        let life = Life::<2>::new(HashSet::<usize>::new(), survival_rules);
        assert_eq!(life, Err(Error::TooHighRule(9, 8)));
    }

    #[test]
    fn test_alive_cells() {
        let mut life = Life::<2>::default();
        assert!(life.alive_cells().is_empty());

        life.alive_cells_mut().insert([0, 0]);
        let mut alive_cells = HashSet::new();
        alive_cells.insert([0, 0]);
        assert_eq!(life.alive_cells(), &alive_cells);
    }

    #[test]
    fn test_get_cell() {
        let mut life = Life::<2>::default();
        life.alive_cells_mut().insert([1, 1]);
        assert!(life.get_cell(&[1, 1]));
        assert!(!life.get_cell(&[0, 0]));
    }

    #[test]
    fn test_set_cell() {
        let mut life = Life::<2>::default();
        life.alive_cells_mut().insert([1, 1]);
        assert!(life.set_cell(&[1, 1], false));
        assert!(life.set_cell(&[0, 0], true));
        assert!(!life.set_cell(&[0, 0], true));
    }

    #[test]
    fn test_toggle_cell() {
        let mut life = Life::<2>::default();
        life.alive_cells_mut().insert([1, 1]);
        life.toggle_cell(&[1, 1]);
        life.toggle_cell(&[0, 0]);
        let expected_alive_cells: HashSet<[i64; 2]> = [[0, 0]].into_iter().collect();
        assert_eq!(life.alive_cells(), &expected_alive_cells);
    }

    #[test]
    fn test_next_generation() {
        let mut life = Life::<2>::default();
        life.alive_cells_mut()
            .extend([[0, 0], [1, 0], [2, 0], [2, 1], [1, 2]]);
        for _ in 0..12 {
            life.next_generation();
        }
        assert_eq!(life.age(), 12);
        let expected_alive_cells: HashSet<[i64; 2]> = [[3, -3], [4, -3], [5, -3], [5, -2], [4, -1]]
            .into_iter()
            .collect();
        assert_eq!(life.alive_cells(), &expected_alive_cells);
    }

    #[test]
    fn test_changed_cells() {
        let mut life = Life::<2>::default();
        life.alive_cells_mut().insert([1, 1]);
        life.next_generation();
        assert_eq!(
            vec![[1, 1]],
            life.changed_cells().copied().collect::<Vec<_>>()
        );
    }
}
