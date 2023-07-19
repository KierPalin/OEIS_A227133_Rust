#![allow(unused)]

use crate::{hca_utils, square_utils, DEPENDENCY_MAPS, GRID_LENGTH, GRID_SIZE, SQUARES_AS_BITLIST};

use itertools::Itertools;
use std::collections::VecDeque;
use std::str;

pub struct State {
    heatmap: Vec<i8>,
    heatmap_peaks: Vec<usize>,
    depth: i8,
}

impl State {
    pub fn new(heatmap: Vec<i8>, heatmap_peaks: Vec<usize>, depth: i8) -> State {
        State {
            heatmap,
            heatmap_peaks,
            depth,
        }
    }

    /**
     * Apply the dependency map to this state.
     * Where each dependency map corresponds to a heatmap_peak
     */
    pub fn get_children(&mut self) -> Vec<State> {
        let mut children: Vec<State> = Vec::new();

        for heatmap_peak in self.heatmap_peaks.iter() {
            let child_heatmap = self.apply_dependency_map(*heatmap_peak);
            let heatmap_peaks = hca_utils::get_peaks(&child_heatmap);
            children.push(State::new(child_heatmap, heatmap_peaks, self.depth - 1))
        }

        children
    }

    /**
     * Does the grid of this state contain a square?
     */
    pub fn contains_squares(&self) -> bool {
        let grid = self.as_grid();

        for square in SQUARES_AS_BITLIST.iter() {
            if (square_utils::square_in_grid(&grid, square)) {
                return true;
            }
        }
        false
    }

    /**
     * Generate a new heatmap by applying the nth dependency_map.
     * The nth element of the heatmap is set to the -1 sentinel value.
     * Cells with a value less than or equal to 0 are not updated.
     * Otherwise, the dependency_map is simply elementwise subtracted from the self.heatmap.
     */
    fn apply_dependency_map(&self, n: usize) -> Vec<i8> {
        let dependency_map = &DEPENDENCY_MAPS[n];
        let mut new_heatmap: Vec<i8> = self
            .heatmap
            .clone()
            .iter()
            .zip(dependency_map)
            .map(|(&heatmap_element, &dependency_map_element)| {
                if heatmap_element <= 0 {
                    heatmap_element
                } else {
                    heatmap_element - dependency_map_element
                }
            })
            .collect();
        new_heatmap[n] = -1;

        new_heatmap
    }

    /**
     * Convert self.heatmap into a grid.
     * All values <= 0 are set to 0; otherwise remain 1.
     */
    fn as_grid(&self) -> Vec<i8> {
        self.heatmap
            .clone()
            .iter()
            .map(|&heatmap_element| (heatmap_element != -1) as i8)
            .collect_vec()
    }

    /**
     * Display the current state & its popcount.
     */
    pub fn print_grid(&self) {
        let grid = self.as_grid();
        println!(
            "F({}) = {}.",
            GRID_LENGTH,
            grid.iter().filter(|&n| *n == 1).count(),
        );

        for row in grid.chunks(GRID_LENGTH as usize) {
            println!("{:?}", row);
        }
        println!();
    }
}

impl Default for State {
    fn default() -> Self {
        let heatmap = hca_utils::get_initial_heatmap();
        let heatmap_peaks = hca_utils::get_peaks(&heatmap);
        Self {
            heatmap,
            heatmap_peaks,
            depth: GRID_SIZE,
        }
    }
}

/**
 * Print out the heatmap.
 */
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_heatmap = "".to_owned();

        for row in self.heatmap.chunks(GRID_LENGTH as usize) {
            formatted_heatmap += &format!("{:?}\n", row);
        }

        write!(f, "Heatmap:\n{}", formatted_heatmap)
    }
}
