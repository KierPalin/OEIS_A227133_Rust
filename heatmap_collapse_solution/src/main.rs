// Remove & recompile when complete:
#![allow(unused)]

use itertools::Itertools;
use lazy_static::lazy_static;

use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

mod state;
use state::*;

mod square_utils;
use square_utils::*;

mod hca_utils;
use hca_utils::*;

lazy_static! {
    static ref SQUARES: Vec<u128> = square_utils::get_squares();
    static ref SQUARES_AS_BITLIST: Vec<Vec<i8>> = square_utils::get_squares_as_bitlist();
    static ref DEPENDENCY_MAPS: Vec<Vec<i8>> = hca_utils::get_dependency_maps();
}

fn search(state_queue: &mut VecDeque<State>, current_depth: u8) -> State {
    let mut next_state_queue: VecDeque<State> = VecDeque::new();
    let mut current_state: State;

    while (!state_queue.is_empty()) {
        current_state = state_queue.pop_front().unwrap();

        // Valid Solution:
        if (current_depth <= SOLUTION_IS_POSSIBLE_DEPTH && !current_state.contains_squares()) {
            println!("Solution found. Queue size is {}", state_queue.len());
            return current_state;
        }
        // Not valid, or not at depth to search yet; just add child states:
        else {
            for child_state in current_state.get_children() {
                next_state_queue.push_back(child_state);
            }
        }
    }
    search(&mut next_state_queue, current_depth - 1)
}

fn main() {
    let now = Instant::now();
    let mut state_queue: VecDeque<State> = VecDeque::from(vec![State::default()]);
    search(&mut state_queue, GRID_SIZE as u8).print_grid();
    println!("Took {:?} to solve.", now.elapsed());
}
