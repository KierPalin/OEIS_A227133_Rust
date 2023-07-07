// Remove & recompile when complete:
#![allow(unused)]

const GRID_LENGTH: u32 = 3;
const GRID_SIZE: u32 = GRID_LENGTH * GRID_LENGTH;

mod square_utils;

/**
 * This is a struct to store pre-calculated values that are neccessary for the search
 *
 */
#[derive(Debug)]
struct GridSolver {
    squares: Vec<u32>, // These binary values are entirely zeros, except for 4 set bits, in the corners of the square.
    two_to_the_power_n_map: Vec<u32>, // Since the values binaries, this enables fast setting & clearing of bits.
    dependency_maps: [u32; GRID_SIZE as usize], // Binary values of how the corners in squares are dependent on one another; applied to heatmaps.
}

impl GridSolver {
    pub fn new() -> GridSolver {
        let squares = square_utils::get_squares();
        let dependency_maps = square_utils::get_dependency_maps(&squares);

        GridSolver {
            squares,
            two_to_the_power_n_map: (0..GRID_SIZE).map(|n| 1 << n).collect(),
            dependency_maps,
        }
    }

    // A heatmap is an encoding of the grid, where each cell contains the number of squares that cell is a part of.
    // The hottest cells are the ones that would be most efficiently set to 0.
    // Multiple cells must be set to

    // 111
    // 111
    // 111

    // 222
    // 242
    // 222

    // 111
    // 111
    // 111

    // 111
    // 131
    // 111

    // 111
    // 101
    // 111

    // 011
    // 121
    // 111

    // 11111
    // 11111
    // 11111
    // 11111
    // 11111

    // 4,4,4,4,4,
    // 4,6,6,6,4,
    // 4,6,8,6,4,
    // 4,6,6,6,4,
    // 4,4,4,4,4

    // 11111
    // 11111
    // 11011
    // 11111
    // 11111

    // 33333
    // 35553
    // 35753
    // 35553
    // 33333

    // 11100
    // 11111
    // 11100
    // 01010
    // 01001

    // 22233
    // 24442
    // 24653
    // 34543
    // 34334

    // 11111
    // 10111
    // 11011
    // 11111
    // 11111

    pub fn search() -> u32 {
        GRID_LENGTH
    }

    // Subtract the dependency map from the heatmap, unset the nth bit:
    fn apply_dependency_map(&self, heat_map: u32, n: u32) -> u32 {
        ((heat_map - self.dependency_maps[n as usize]) & !(1 << n))
    }
}

fn main() {
    let grid_solver = GridSolver::new();

    for dependency_map in grid_solver.dependency_maps {
        println!("\n {:09b}", dependency_map);
    }
}
