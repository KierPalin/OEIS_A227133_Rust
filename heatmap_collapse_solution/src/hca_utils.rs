#![allow(unused)]

use std::collections::VecDeque;

use std::collections::HashSet;

use itertools::Itertools;

use crate::{square_utils, GRID_LENGTH, GRID_SIZE};
use crate::{DEPENDENCY_MAPS, SQUARES, SQUARES_AS_BITLIST};

//------------------------------------
// Heatmap & Dependency map Functions:
//------------------------------------

/**
 * The heatmap represents the most commonly used cells by the squares.
 * Hence adding all of the squares together will yield the initial heatmap.
 *
 * Setting cells that have the highest values (the 'hottest', 'peaks') is the most
 * efficient use of the 0 tiles; since it eliminates the most number of squares from the grid.
 */
pub fn get_initial_heatmap() -> Vec<i8> {
    SQUARES_AS_BITLIST
        .iter()
        .fold(vec![0; GRID_SIZE as usize], |heatmap, square| {
            add_lists(&heatmap, square)
        })
}

/**
 * Get a vector of dependency_maps.
 * Where the nth dependency_map is a vector of i8, of equal length to a grid and heatmap;
 * Where the all squares that have an nth bit set are bitwise OR'd together.
 * This creates a map of what squares are affected if the nth cell in the heatmap is cleared.
 *
 * Hence subtracting a dependency_map n from a heatmap is the same as decrementing all cells that
 * are corners to any square that uses cell n.
 *
 * These dependency_maps can be treated as constants. Hence they're stored lazily; see main.
 */
pub fn get_dependency_maps() -> Vec<Vec<i8>> {
    // All bits are clear, except nth bit is set:
    let bit_masks: Vec<u128> = (0..(GRID_SIZE as u128)).map(|n| 1 << n).collect();

    let mut dependency_maps: [u128; GRID_SIZE as usize] = [0; GRID_SIZE as usize];

    // Get the dependency map by Bitwise ORing all squares with that have a 1 in their nth bit:
    for n in 0..(GRID_SIZE as usize) {
        dependency_maps[n] = SQUARES
            .iter()
            .filter(|&&square| (square & bit_masks[n]) == bit_masks[n]) // Is the nth bit set?
            .fold(0, |acc, square| acc | square); // Accumulate them via Bitwise OR
    }

    // Return these maps as bitlists:
    dependency_maps
        .iter()
        .map(|dependency_map| square_utils::get_bitlist(*dependency_map))
        .collect()
}

/**
 * Get a vector of all the indices that are equal to the maximum value in the heatmap.
 * These peaks are the 'hottest' points - they are the cells most used by squares in the grid.
 *
 * Thus setting them to 0 first, makes efficient use of 0 tiles;
 * Since it eliminates the most number of squares.
 */
pub fn get_peaks(heatmap: &[i8]) -> Vec<usize> {
    let maximum_value = heatmap.iter().max().unwrap();
    heatmap
        .iter()
        .enumerate()
        .filter_map(|(index, &cell)| {
            if cell == *maximum_value {
                Some(index)
            } else {
                None
            }
        })
        .collect()
}

//-----------------
// Private Helpers:
//-----------------

/**
 * Simply elementwise add the heatmap with the square
 */
fn add_lists(heatmap: &[i8], square: &Vec<i8>) -> Vec<i8> {
    heatmap
        .iter()
        .zip(square)
        .map(|(heatmap_bit, square_bit)| heatmap_bit + square_bit)
        .collect()
}
