#![allow(unused)]

use std::collections::VecDeque;

use std::collections::HashSet;

use itertools::Itertools;

use crate::{square_utils, GRID_LENGTH, GRID_SIZE};
use crate::{DEPENDENCY_MAPS, SQUARES, SQUARES_AS_BITLIST};

//------------------------------------
// Heatmap & Dependency map Functions:
//------------------------------------

pub fn get_heatmap() -> Vec<i8> {
    SQUARES_AS_BITLIST.iter().fold(
        vec![0; square_utils::GRID_SIZE as usize],
        |heatmap, square| add_lists(&heatmap, square),
    )
}

pub fn get_dependency_maps() -> Vec<Vec<i8>> {
    // All bits are clear, except nth bit is set:
    let bit_masks: Vec<u128> = (0..(square_utils::GRID_SIZE as u128))
        .map(|n| 1 << n)
        .collect();

    let mut dependency_maps: [u128; square_utils::GRID_SIZE as usize] =
        [0; square_utils::GRID_SIZE as usize];

    // Get the dependency map by Bitwise ORing all squares with that have a 1 in their nth bit:
    for n in 0..(square_utils::GRID_SIZE as usize) {
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

/**
 * All heatmaps will have their peaks in the same positions.
 * Even grids will have 4 peaks in the innermost square.
 * Odd grids will have a single peak in the centre.
 */
// pub fn get_initial_heatmap_peak_indices() -> Vec<i8> {
//     if (GRID_SIZE % 2 == 0) {
//         let inner_square_top_left_corner =
//             (GRID_LENGTH * ((GRID_LENGTH / 2) - 1)) + (GRID_LENGTH / 2) - 1;
//         let inner_square_top_right_corner = inner_square_top_left_corner + 1;
//         let inner_square_bot_left_corner = inner_square_top_left_corner + GRID_LENGTH;
//         let inner_square_bot_right_corner = inner_square_bot_left_corner + 1;

//         // Accommodate for 0-based indexing:
//         vec![
//             inner_square_top_left_corner,
//             inner_square_top_right_corner,
//             inner_square_bot_left_corner,
//             inner_square_bot_right_corner,
//         ]
//     } else {
//         vec![((GRID_SIZE + 1) / 2) - 1]
//     }
// }

/**
 * Get central indices
 * Simpler to retrieve all indices of elements that are the maximum.
 */

// pub fn get_initial_heatmap_peak_queue() -> VecDeque<i8> {
//     let mut queue = HashSet::new();
//     let mut tl_corner: i8 = (GRID_LENGTH * ((GRID_LENGTH / 2) - 1)) + (GRID_LENGTH / 2) - 1;
//     let mut scale: i8 = 2;

//     while (tl_corner != 0) {
//         for i in (tl_corner..(tl_corner + ((scale - 1) * GRID_LENGTH) + scale - 1)) {
//             queue.insert(i);
//         }

//         tl_corner -= GRID_LENGTH - 1;
//         scale += 1;
//     }
//     println!("Queue: {:?}", queue.iter().collect_vec());
//     VecDeque::from(queue.into_iter().collect_vec())
// }

//------------------------------------------
// Reflection & Rotation Checking Functions:
//------------------------------------------

// pub struct ReflectionAndRotationLookupTable {
//     is_minimum_rotation: Vec<bool>,
//     is_minimum_reflection: Vec<bool>,
// }

// impl ReflectionAndRotationLookupTable {
//     pub fn new() -> ReflectionAndRotationLookupTable {
//         ReflectionAndRotationLookupTable {
//             is_minimum_rotation: (0..GRID_SIZE).map(|x| x < ).collect(),
//             is_minimum_reflection: ()
//         }
//     }
// }

// [1, 2, 3, 4, 5
// ,6, 7, 8, 9, A
// ,B, C, D, 9, 1
// ,3, 5, 5, 3, 1
// ,3, 3, 3, 3, 1]

/**
 * Generate a table
 */

//-----------------
// Private Helpers:
//-----------------

fn add_lists(heatmap: &[i8], square: &Vec<i8>) -> Vec<i8> {
    heatmap
        .iter()
        .zip(square)
        .map(|(heatmap_bit, square_bit)| heatmap_bit + square_bit)
        .collect()
}
