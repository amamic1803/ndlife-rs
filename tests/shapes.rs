//! Test various shapes in the Conway's Game of Life

use ndlife::conways_game_of_life;
use std::borrow::Borrow;
use std::collections::HashSet;

/// Tests an initial shape in the Conway's Game of Life by running it for a number of rounds and checking the final state.
fn test_shape<T, U, V, W>(initial_state: T, final_state: V, rounds: i64)
where
    T: IntoIterator<Item = U>,
    U: Borrow<[i64; 2]>,
    V: IntoIterator<Item = W>,
    W: Borrow<[i64; 2]>,
{
    let mut life = conways_game_of_life();
    for cell in initial_state.into_iter() {
        let cell = cell.borrow();
        life.set_cell(cell, true);
    }

    (0..rounds).for_each(|_| life.next_generation());

    assert_eq!(life.age(), rounds);

    let final_state: HashSet<[i64; 2]> = final_state.into_iter().map(|c| *c.borrow()).collect();
    assert_eq!(life.alive_cells(), &final_state);
}

// still lives

#[test]
fn test_block() {
    let initial_state = [[0, 0], [0, 1], [1, 0], [1, 1]];
    let final_state = [[0, 0], [0, 1], [1, 0], [1, 1]];
    test_shape(initial_state, final_state, 100);
}

#[test]
fn test_beehive() {
    let initial_state = [[0, 1], [1, 0], [1, 2], [2, 0], [2, 2], [3, 1]];
    let final_state = [[0, 1], [1, 0], [1, 2], [2, 0], [2, 2], [3, 1]];
    test_shape(initial_state, final_state, 100);
}

#[test]
fn test_loaf() {
    let initial_state = [[0, 0], [1, 1], [2, 1], [3, 0], [3, -1], [2, -2], [1, -1]];
    let final_state = [[0, 0], [1, 1], [2, 1], [3, 0], [3, -1], [2, -2], [1, -1]];
    test_shape(initial_state, final_state, 100);
}

#[test]
fn test_boat() {
    let initial_state = [[0, 0], [0, 1], [1, 1], [2, 0], [1, -1]];
    let final_state = [[0, 0], [0, 1], [1, 1], [2, 0], [1, -1]];
    test_shape(initial_state, final_state, 100);
}

#[test]
fn test_tub() {
    let initial_state = [[0, 0], [1, 1], [2, 0], [1, -1]];
    let final_state = [[0, 0], [1, 1], [2, 0], [1, -1]];
    test_shape(initial_state, final_state, 1);
}

// oscillators

#[test]
fn test_blinker() {
    let initial_state = [[0, 0], [0, 1], [0, 2]];
    let final_state = [[-1, 1], [0, 1], [1, 1]];
    test_shape(initial_state, final_state, 1);

    let initial_state = [[0, 0], [0, 1], [0, 2]];
    let final_state = [[0, 0], [0, 1], [0, 2]];
    test_shape(initial_state, final_state, 2);
}

#[test]
fn test_toad() {
    let initial_state = [[0, 0], [1, 0], [2, 0], [1, 1], [2, 1], [3, 1]];
    let final_state = [[0, 0], [0, 1], [1, -1], [2, 2], [3, 0], [3, 1]];
    test_shape(initial_state, final_state, 1);

    let initial_state = [[0, 0], [1, 0], [2, 0], [1, 1], [2, 1], [3, 1]];
    let final_state = [[0, 0], [1, 0], [2, 0], [1, 1], [2, 1], [3, 1]];
    test_shape(initial_state, final_state, 2);
}

#[test]
fn test_beacon() {
    let initial_state = [
        [0, 0],
        [0, 1],
        [1, 0],
        [1, 1],
        [2, -1],
        [2, -2],
        [3, -1],
        [3, -2],
    ];
    let final_state = [[0, 0], [0, 1], [1, 1], [2, -2], [3, -1], [3, -2]];
    test_shape(initial_state, final_state, 1);

    let initial_state = [
        [0, 0],
        [0, 1],
        [1, 0],
        [1, 1],
        [2, -1],
        [2, -2],
        [3, -1],
        [3, -2],
    ];
    let final_state = [
        [0, 0],
        [0, 1],
        [1, 0],
        [1, 1],
        [2, -1],
        [2, -2],
        [3, -1],
        [3, -2],
    ];
    test_shape(initial_state, final_state, 2);
}

#[test]
fn test_pulsar() {
    let initial_state = [
        [1, 2],
        [1, 3],
        [1, 4],
        [6, 2],
        [6, 3],
        [6, 4],
        [2, 1],
        [3, 1],
        [4, 1],
        [2, 6],
        [3, 6],
        [4, 6],
        [1, -2],
        [1, -3],
        [1, -4],
        [6, -2],
        [6, -3],
        [6, -4],
        [2, -1],
        [3, -1],
        [4, -1],
        [2, -6],
        [3, -6],
        [4, -6],
        [-1, -2],
        [-1, -3],
        [-1, -4],
        [-6, -2],
        [-6, -3],
        [-6, -4],
        [-2, -1],
        [-3, -1],
        [-4, -1],
        [-2, -6],
        [-3, -6],
        [-4, -6],
        [-1, 2],
        [-1, 3],
        [-1, 4],
        [-6, 2],
        [-6, 3],
        [-6, 4],
        [-2, 1],
        [-3, 1],
        [-4, 1],
        [-2, 6],
        [-3, 6],
        [-4, 6],
    ];

    let final_state1 = [
        [1, 2],
        [1, 3],
        [2, 1],
        [3, 1],
        [2, 3],
        [3, 2],
        [2, 5],
        [3, 5],
        [3, 6],
        [3, 7],
        [5, 2],
        [5, 3],
        [6, 3],
        [7, 3],
        [1, -2],
        [1, -3],
        [2, -1],
        [3, -1],
        [2, -3],
        [3, -2],
        [2, -5],
        [3, -5],
        [3, -6],
        [3, -7],
        [5, -2],
        [5, -3],
        [6, -3],
        [7, -3],
        [-1, -2],
        [-1, -3],
        [-2, -1],
        [-3, -1],
        [-2, -3],
        [-3, -2],
        [-2, -5],
        [-3, -5],
        [-3, -6],
        [-3, -7],
        [-5, -2],
        [-5, -3],
        [-6, -3],
        [-7, -3],
        [-1, 2],
        [-1, 3],
        [-2, 1],
        [-3, 1],
        [-2, 3],
        [-3, 2],
        [-2, 5],
        [-3, 5],
        [-3, 6],
        [-3, 7],
        [-5, 2],
        [-5, 3],
        [-6, 3],
        [-7, 3],
    ];

    let final_state2 = [
        [1, 2],
        [1, 3],
        [1, 4],
        [2, 3],
        [3, 4],
        [3, 5],
        [3, 6],
        [2, 5],
        [4, 6],
        [2, 1],
        [3, 1],
        [4, 1],
        [3, 2],
        [4, 3],
        [5, 3],
        [6, 3],
        [5, 2],
        [6, 4],
        [1, -2],
        [1, -3],
        [1, -4],
        [2, -3],
        [3, -4],
        [3, -5],
        [3, -6],
        [2, -5],
        [4, -6],
        [2, -1],
        [3, -1],
        [4, -1],
        [3, -2],
        [4, -3],
        [5, -3],
        [6, -3],
        [5, -2],
        [6, -4],
        [-1, -2],
        [-1, -3],
        [-1, -4],
        [-2, -3],
        [-3, -4],
        [-3, -5],
        [-3, -6],
        [-2, -5],
        [-4, -6],
        [-2, -1],
        [-3, -1],
        [-4, -1],
        [-3, -2],
        [-4, -3],
        [-5, -3],
        [-6, -3],
        [-5, -2],
        [-6, -4],
        [-1, 2],
        [-1, 3],
        [-1, 4],
        [-2, 3],
        [-3, 4],
        [-3, 5],
        [-3, 6],
        [-2, 5],
        [-4, 6],
        [-2, 1],
        [-3, 1],
        [-4, 1],
        [-3, 2],
        [-4, 3],
        [-5, 3],
        [-6, 3],
        [-5, 2],
        [-6, 4],
    ];

    let final_state3 = initial_state;

    test_shape(initial_state, final_state1, 1);
    test_shape(initial_state, final_state2, 2);
    test_shape(initial_state, final_state3, 3);
}

// spaceships

#[test]
fn test_glider() {
    let initial_state = [[0, 0], [1, 0], [2, 0], [2, 1], [1, 2]];

    let final_state1 = [[0, 1], [1, 0], [1, -1], [2, 0], [2, 1]];
    let final_state2 = [[0, 0], [2, -1], [1, -1], [2, 0], [2, 1]];
    let final_state3 = [[1, -1], [1, 1], [2, 0], [2, -1], [3, 0]];
    let final_state4 = [[1, -1], [2, -1], [3, -1], [3, 0], [2, 1]];

    test_shape(initial_state, final_state1, 1);
    test_shape(initial_state, final_state2, 2);
    test_shape(initial_state, final_state3, 3);
    test_shape(initial_state, final_state4, 4);
}

#[test]
fn test_lightweight_spaceship() {
    let initial_state = [
        [0, 1],
        [0, 3],
        [1, 0],
        [2, 0],
        [3, 0],
        [4, 0],
        [4, 1],
        [4, 2],
        [3, 3],
    ];

    let final_state1 = [
        [1, 0],
        [2, 0],
        [3, 0],
        [4, 0],
        [4, 1],
        [4, 2],
        [3, 2],
        [1, 1],
        [2, 1],
        [2, -1],
        [3, -1],
        [5, 1],
    ];
    let final_state2 = [
        [1, 1],
        [1, -1],
        [4, -1],
        [2, 2],
        [3, 2],
        [4, 2],
        [5, 2],
        [5, 0],
        [5, 1],
    ];
    let final_state3 = [
        [4, 0],
        [5, 0],
        [2, 1],
        [3, 1],
        [5, 1],
        [6, 1],
        [2, 2],
        [3, 2],
        [4, 2],
        [5, 2],
        [4, 3],
        [3, 3],
    ];
    let final_state4 = [
        [2, 1],
        [2, 3],
        [3, 0],
        [4, 0],
        [5, 0],
        [6, 0],
        [6, 1],
        [6, 2],
        [5, 3],
    ];

    test_shape(initial_state, final_state1, 1);
    test_shape(initial_state, final_state2, 2);
    test_shape(initial_state, final_state3, 3);
    test_shape(initial_state, final_state4, 4);
}
