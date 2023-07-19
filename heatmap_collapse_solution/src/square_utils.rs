use crate::{GRID_LENGTH, GRID_SIZE, SQUARES};

extern crate itertools;
use itertools::iproduct; // used by get_squares.

extern crate unfold;

//-----------------------------
// Square Generation Functions:
//-----------------------------

/**
 * All possible squares in the grid can be pre-calculated.
 * These can then be checked against a candidate_grid via bitwise anding.
 *
 * Stored lazily: see SQUARES in main.
 */
pub fn get_squares() -> Vec<u128> {
    let grid_length = GRID_LENGTH as i32;
    let grid_size = GRID_SIZE as i32;
    let squares: Vec<u128> = iproduct!(1..=grid_size, 2..=grid_length)
        .filter(|(index, scale)| valid_square(*index, *scale))
        .map(|(index, scale)| construct_square(index, scale))
        .collect();
    squares
}

/**
 * A square is an integer of entirely zeroes, except for 4 set bits,
 * These 4 set bits are the corners of the square.
 */
fn construct_square(top_left_corner_index: i32, scale: i32) -> u128 {
    let tl_corner: u32 = top_left_corner_index.try_into().unwrap();
    let scale: u32 = scale.try_into().unwrap();
    let grid_length = GRID_LENGTH as u32;

    let square: u128 = u128::pow(2, tl_corner - 1)
        + u128::pow(2, tl_corner - 1 + scale - 1)
        + u128::pow(2, tl_corner - 1 + (grid_length * (scale - 1)))
        + u128::pow(2, tl_corner - 1 + (grid_length * (scale - 1)) + scale - 1);
    square
}

//------------------------------------
// Square Generation Helper Functions:
//------------------------------------

pub fn valid_square(index: i32, scale: i32) -> bool {
    square_within_bounds(index, scale) & !edge_is_on_different_row(index, scale)
}

fn get_current_row(index: i32) -> i32 {
    let grid_length = GRID_LENGTH as i32;
    ((index - 1) + grid_length - ((index - 1) % grid_length)) / grid_length
}

fn edge_is_on_different_row(index: i32, scale: i32) -> bool {
    (get_current_row(index + scale - 1) - get_current_row(index)) > 0
}

fn square_within_bounds(index: i32, scale: i32) -> bool {
    let grid_length = GRID_LENGTH as i32;
    let grid_size = GRID_SIZE as i32;
    (index + (scale - 1) + grid_length * (scale - 1)) <= grid_size
}

//------------------------
// Grid Checking Function:
//------------------------

/**
 * Bitwise and the grid & the square.
 * If the result is equal to the square then the grid neccessarily contains that square.
 */
pub fn square_in_grid(grid: &[i8], square: &Vec<i8>) -> bool {
    grid.iter()
        .zip(square)
        .map(|(&grid_bit, square_bit)| grid_bit & square_bit)
        .collect::<Vec<i8>>()
        == *square
}

//---------------------------------
// Miscellaneous Utility Functions:
//---------------------------------

pub fn get_bitlist(input: u128) -> Vec<i8> {
    (0..GRID_SIZE).map(|x| ((input >> x) & 1) as i8).collect()
}

pub fn get_squares_as_bitlist() -> Vec<Vec<i8>> {
    SQUARES.iter().map(|&square| get_bitlist(square)).collect()
}
