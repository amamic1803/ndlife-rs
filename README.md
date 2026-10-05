# ndlife

[<img alt="GitHub Repository Static Badge" src="https://img.shields.io/badge/GitHub-ndlife-blue?logo=github">](https://github.com/amamic1803/ndlife-rs)
[<img alt="Crates.io Version" src="https://img.shields.io/crates/v/ndlife?logo=rust">](https://crates.io/crates/ndlife)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/ndlife?logo=docs.rs&label=docs.rs">](https://docs.rs/ndlife)
[<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/amamic1803/ndlife-rs/test.yml">](https://github.com/amamic1803/ndlife-rs/actions/workflows/test.yml)
[<img alt="GitHub License" src="https://img.shields.io/github/license/amamic1803/ndlife-rs">](https://github.com/amamic1803/ndlife-rs/blob/main/LICENSE)

***ndlife*** is an implementation of infinite, N-dimensional game of life in Rust.

The game of life is a cellular automaton devised by the British mathematician _John Horton Conway_ in 1970.
The game is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input.
One interacts with the Game of Life by creating an initial configuration and observing how it evolves.
This crate extends the game of life to N dimensions, where N is any positive integer.

### Example
```rust
use std::collections::HashSet;
use ndlife::Life;

// setup conway's game of life
let birth_rules = [3];
let survival_rules = [2, 3];
let mut life = Life::<2>::new(birth_rules, survival_rules).unwrap();

// or use shortcut
// let mut life = conways_game_of_life();

// glider pattern
life.alive_cells_mut().extend([[0, 0], [1, 0], [2, 0], [2, 1], [1, 2]]);

// advance life by 4 generations (repeat cycle for glider)
for _ in 0..4 {
   life.next_generation();
}

// glider moves one cell diagonally (right-down) every 4 generations
let mut expected_alive_cells = HashSet::with_capacity(5);
expected_alive_cells.extend([[1, -1], [2, -1], [3, -1], [3, 0], [2, 1]]);

// assert that is indeed what happened
assert_eq!(life.alive_cells(), &expected_alive_cells);
```

## No-std support
There is currently no support for `no-std` environments.

## WebAssembly support
WASM targets should work without any additional configuration.

## License
This project is licensed under the [MIT License](https://github.com/amamic1803/ndlife-rs/blob/main/LICENSE).

## Contributing
Contributions are welcome!

Please open an issue or a pull request in the [GitHub repository](https://github.com/amamic1803/ndlife-rs).
