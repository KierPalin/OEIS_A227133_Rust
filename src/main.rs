// Remove & recompile when complete:
#![allow(unused)]

const GRID_LENGTH: u32 = 3;
const GRID_SIZE: u32 = GRID_LENGTH * GRID_LENGTH;

mod square_utils;

#[derive(Debug)]
struct GridSolver {
    squares: Vec<u32>,                //
    two_to_the_power_n_map: Vec<u32>, //
    dependency_maps: [u32; GRID_SIZE as usize],
}

impl GridSolver {
    fn new() -> GridSolver {
        let squares = square_utils::get_squares();
        let dependency_maps = square_utils::get_dependency_maps(&squares);

        GridSolver {
            squares,
            two_to_the_power_n_map: (0..GRID_SIZE).map(|n| 1 << n).collect(),
            dependency_maps,
        }
    }

    // Subtract the dependency map from the heatmap, unset the nth bit:
    fn apply_dependency_map(&self, heat_map: u32, n: u32) -> u32 {
        ((heat_map - self.dependency_maps[n as usize]) & !(1 << n))
    }
}

fn main() {
    let grid_solver = GridSolver::new();
    println!("Result:\n {:?}", grid_solver.dependency_maps);
}
