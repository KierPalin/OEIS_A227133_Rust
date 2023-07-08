# OEIS_A227133_Rust
A Solution to the OEIS/A227133 Problem, in pure Rust.

Given a square grid with side n consisting of n^2 cells (or points), a(n) is the maximum number of points that can be painted so that no four of the painted ones form a square with sides parallel to the grid.

https://oeis.org/A227133

## **Current Workload:**
There is a substantial optimisation potential involving the generation and validation of grids.

1. Problem 1: The validation of grids is presently very fast - due to the par_iter(), find_any() iterable methods from the rayon crate and the bitwise operation algorithm for searching for squares in grids.

The present issue is the generation of grids. Grids are generated as permutations via Gosper's hack. This allows the search to be optimised, since it ensures that only the grids that have a greater popcount than the solution are searched - greatly reducing the domain down from all possible grids. While Gosper's hack is a fast algorithm the scale of the integers that are in a popcount is immense, hence generation can still take some time and, crucially, a significant proportion of memory to store.

One solution to this memory isssue is checking each grid as it is generated - via .filter() - which will ensure that each grid is checked as it is generated, removing the need to store any permutations other than the one being proccessed. The downside to this is that the checking algorithm must be sequential in this implementation.

To ameliorate these issues the permutations are generated in chunks, and then these chunks are checked in parallel.

At present testing, for N = 6, checking an entire popcount of grids is extremely fast, and the permutation generation is the asymptotic factor.


One solution to this issue is to split the generatin of permutations in Gosper's hack into sections, where K threads will each own a generator for the grids, they can then filter them sequentially.


This is an elegant solution to this optimisation issue in the naive_solution. It also allows for further optimisations - since the gosper's hack and grid checking functions may be combined.


Each generator will be seeded with an equally spaced permutation, such that the output set is the same, but it has been generated from K unique inputs.


2.  Problem 2: Since the grids are squares, each grid has 3 other rotations. One significant optimisation would be to skip over these grids. This is at first a difficult issue - since there is a signficiant overhead with rotating large grids. Additionally, communication between multiple threads about which grids to check will be difficult.


One policy, that does not require thread communication, is to only check the grids that represent the smallest possible integer out of any of their rotations.

Ideally this would occur during permutation generation:

Permutatios in Gosper's hack can be tested via bitmask: 

The location of the rotation of a bit can be pre-calculated and stored in a lookup table, then:

If bit n, when set to a 0, is smaller than all rotated n: the permutation should be generated.

## **Currently Operational:**
naive_solution

run via: cargo run --release


## **TODO:**
1. Implement a parllelised version of the naive_solution
2. Neaten code via structures.
3. Implement the heatmap collapse algorithm