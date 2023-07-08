# OEIS_A227133_Rust
A Solution to the OEIS/A227133 Problem, in pure Rust.

Given a square grid with side n consisting of n^2 cells (or points), a(n) is the maximum number of points that can be painted so that no four of the painted ones form a square with sides parallel to the grid.

https://oeis.org/A227133

## **Currently Operational:**
naive_solution

run via: cargo run --release


## **TODO:**
1. Implement a parllelised version of the naive_solution
2. Neaten code via structures.
3. Implement the heatmap collapse algorithm