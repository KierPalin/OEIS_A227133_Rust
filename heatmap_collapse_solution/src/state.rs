#![allow(unused)]

use std::collections::VecDeque;
use std::str;

use itertools::Itertools;

use crate::{hca_utils, square_utils, DEPENDENCY_MAPS, GRID_LENGTH, GRID_SIZE, SQUARES_AS_BITLIST};

pub struct State {
    pub heatmap: Vec<i8>,
    pub heatmap_peaks: Vec<usize>,
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

    pub fn get_children(&mut self) -> Vec<State> {
        let mut children: Vec<State> = Vec::new();

        for heatmap_peak in self.heatmap_peaks.iter() {
            let child_heatmap = self.apply_dependency_map(*heatmap_peak);
            let heatmap_peaks = hca_utils::get_peaks(&child_heatmap);
            children.push(State::new(child_heatmap, heatmap_peaks, self.depth - 1))
        }

        children
    }

    pub fn contains_squares(&self) -> bool {
        let grid = self.as_grid();

        // println!("Checking for squares on grid: {:?}", grid);

        for square in SQUARES_AS_BITLIST.iter() {
            if (square_utils::square_in_grid(&grid, square)) {
                return true;
            }
        }
        false
    }

    //------------------------------------
    // Heatmap Selection Policy Functions:
    //------------------------------------
    fn apply_dependency_map(&self, n: usize) -> Vec<i8> {
        let dependency_map = &DEPENDENCY_MAPS[n];

        // println!(
        //     "Applying dependency map: {:?} to {:?}, via n = {}:",
        //     dependency_map, self.heatmap, n
        // );

        let mut new_heatmap: Vec<i8> = self
            // .heatmap
            // .clone()
            // .iter()
            // .filter(|&&heatmap_element| heatmap_element != 0)
            // .zip(dependency_map)
            // .map(|(&heatmap_element, &dependency_map_element)| {
            //     heatmap_element - dependency_map_element
            // })
            // .collect();
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

        // println!("NewHeatmap: {:?}\n", new_heatmap);
        new_heatmap[n] = -1;

        // println!("With n set to -1: {:?}\n\n", new_heatmap);

        new_heatmap
    }

    fn as_grid(&self) -> Vec<i8> {
        self.heatmap
            .clone()
            .iter()
            .map(|&heatmap_element| (heatmap_element != -1) as i8)
            .collect_vec()
    }

    pub fn print_grid(&self) {
        let grid = self.as_grid();
        println!(
            "F({}) = {}, with peaks: {:?}",
            GRID_LENGTH,
            grid.iter().filter(|&n| *n == 1).count(),
            self.heatmap_peaks
        );

        for row in grid.chunks(GRID_LENGTH as usize) {
            println!("{:?}", row);
        }
        println!();
    }
}

impl Default for State {
    fn default() -> Self {
        let heatmap = hca_utils::get_heatmap();
        let heatmap_peaks = hca_utils::get_peaks(&heatmap);
        Self {
            heatmap,
            heatmap_peaks,
            depth: square_utils::GRID_SIZE,
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_heatmap = "".to_owned();

        for row in self.heatmap.chunks(GRID_LENGTH as usize) {
            formatted_heatmap += &format!("{:?}\n", row);
        }

        write!(f, "Heatmap:\n{}", formatted_heatmap)
    }
}
