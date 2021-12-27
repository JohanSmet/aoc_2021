# Advent of Code

My solutions for [Advent Of Code 2021](https://adventofcode.com/2021).

Solutions are written in either C++ or Rust.

Solutions written in C++ should compile with any recent C++20 compatible compiler.
Simple Makefiles are provided, I've only tested these on Linux.

This was my first experience with Rust so the solutions are not the cleanest or nicest
Rust code possible. The first goal was always to find the quickest path to a solution,
learning Rust along the way was "only" the second goal. At least clippy doesn't complain
anywhere (and I only silenced a warning once ;-)).

Only day 18 is missing a solution in Rust. My understanding of Rust's memory model wasn't
sufficient to write a tree-based solution without pulling out what's left of my hair. 
While working on a "flat" solution I grew sufficiently frustrated and switched to C++ to
bang out a quick tree-based solution to avoid falling to far behind.
