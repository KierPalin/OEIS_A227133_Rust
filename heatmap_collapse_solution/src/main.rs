/**
 * Author: Kier Palin.
 *
 * A solution to the OEIS/A227133 problem.
 *
 * Where 1 is the tile to maximise.
 * A 'unit square' is a grid that contains only the 1 tiles in its corners, all else are 0;
 * hence representing an atomic, illegal, state for a grid.
 *
 * These can be combined together via Bitwise ORing all grids that have their nth bit set.
 * This will create a 'dependency_map'; outlining the relationship between a cell and
 * the corners of the squares that use it (depend upon it).
 * These dependency_maps can be pre-calculated & make checking a grid for squares trivial (simple bitwise AND).
 *
 * Basic Algorithm:
 *
 * A grid can be modelled 3-dimensionally:
 * Where the 3rd dimension is a list of the corners of the unit squares that overlap on each cell.
 * Since all corners of the squares are represented as 1 tiles this 3rd dimension can simply be expressed as a count.
 * All of the 1's that repsent the corners of the possible squares accumulated.
 *
 * The peaks in this heatmap are the cells with the greatest value; the 'hottest' ones;
 * They are the cells that are used as corners by the most number of squares;
 * Hence setting the hottest cell to a 0 is neccessarily the most efficient use of the 0 tiles.
 * There are multiple, distinct, solutions to most grids, since the most efficient usage of the 0 tile is also relative
 * to the previously used tiles.
 *
 * Since the search is for the grid with the greatest number of 1 tiles: grid checking should start from the top down.
 * Where the 'top' are the grids with the most number of 1 tiles; the quantity of 1 tiles in a grid is its 'popcount'
 * Hence the problem can be approached as breadth-first search; where all grids with popcount k are checked,
 * before moving onto all grids with popcount k-1; these grids are the children of the above grids & can be calculated via dependency maps.
 *
 * However, since it is impossible for a grid to contain > (GRID_SIZE - GRID_LENGTH + 1) number of 1 tiles;
 * since there would be too few 0 tiles to fill the squares that form along a diagonal - it is not neccessary to check popcounts above this threshold.
 */
mod state;
use state::*;

mod square_utils;

mod hca_utils;
use lazy_static::lazy_static;

use std::{collections::VecDeque, time::Instant};

pub const GRID_LENGTH: i8 = 3;
pub const GRID_SIZE: i8 = GRID_LENGTH * GRID_LENGTH;
pub const SOLUTION_IS_POSSIBLE_DEPTH: u8 = (GRID_SIZE - GRID_LENGTH + 1) as u8;

lazy_static! {
    static ref SQUARES: Vec<u128> = square_utils::get_squares();
    static ref SQUARES_AS_BITLIST: Vec<Vec<i8>> = square_utils::get_squares_as_bitlist();
    static ref DEPENDENCY_MAPS: Vec<Vec<i8>> = hca_utils::get_dependency_maps();
}

fn search(state_queue: &mut VecDeque<State>, current_depth: u8) -> State {
    let mut next_state_queue: VecDeque<State> = VecDeque::new();
    let mut current_state: State;

    while !state_queue.is_empty() {
        current_state = state_queue.pop_front().unwrap();

        // Valid Solution:
        if current_depth <= SOLUTION_IS_POSSIBLE_DEPTH && !current_state.contains_squares() {
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
