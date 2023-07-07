// Remove & recompile when complete:
#![allow(unused)]

use crate::GRID_LENGTH;
use crate::GRID_SIZE;

extern crate itertools;
use itertools::iproduct;


// Generate all of the squares possible for this:
pub fn get_squares() -> Vec<u32> {
    let squares: Vec<u32> = 
        iproduct!(1..=GRID_SIZE, 2..=GRID_LENGTH)
        .into_iter()
        .filter(|(index, scale)| valid_square(*index, *scale))
        .map(|(index, scale)| construct_square(index, scale))
        .collect();
    return squares;
}

pub fn valid_square(index: u32, scale: u32) -> bool {
    return square_within_bounds(index, scale) & ! edge_is_on_different_row(index, scale);
}

pub fn grid_contains_any_squares(grid: u32, squares: Vec<u32>) -> bool {
    for square in squares {
        if ((grid & square) == square) {
            return true;
        }
    }
    return false;
}


pub fn get_dependency_maps(squares: &Vec<u32>) -> [u32; GRID_SIZE as usize] {    
    let bit_masks: Vec<u32> = (0..GRID_SIZE).map(|n| 1 << n).collect(); // All bits are clear, except nth bit is set

    let mut dependency_maps: [u32; GRID_SIZE as usize] = [0; GRID_SIZE as usize];

    // A single u32 of all the squares that have a set nth bit, Bitwise ORd together:
    for n in 0..(GRID_SIZE as usize) {
        dependency_maps[n] = 
            squares
            .into_iter()
            .filter(|&square| (square & bit_masks[n]) == bit_masks[n])  // Is the nth bit set?
            .fold(0, |acc, square| acc | square);                       // Accumulate them via Bitwise OR 
    }

    return dependency_maps;
}

fn construct_square(top_left_corner_index: u32, scale: u32) -> u32 {
    let square: u32 =
        u32::pow(2, top_left_corner_index - 1) +
        u32::pow(2, top_left_corner_index - 1 + scale - 1) +
        u32::pow(2, top_left_corner_index - 1 + (GRID_LENGTH * (scale - 1))) +
        u32::pow(2, top_left_corner_index - 1 + (GRID_LENGTH * (scale - 1)) + scale - 1);
    return square;
}

fn get_current_row(index: u32) -> u32 {
    return ((index - 1) + GRID_LENGTH - ((index - 1) % GRID_LENGTH)) / GRID_LENGTH; // -1 since this requires 0-based indexing
}

fn edge_is_on_different_row(index: u32, scale: u32) -> bool {
    return (get_current_row(index + scale - 1) - get_current_row(index)) > 0;
}

fn square_within_bounds(index: u32, scale: u32) -> bool {
    return (index + (scale - 1) + GRID_LENGTH * (scale - 1)) <= GRID_SIZE;
}