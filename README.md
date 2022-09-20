# CUS1188 Sorting Algorithms
This project contains implementations of the insertion sort and selection sort algorithms
in the Rust programming language. I had to time the execution of each implementation
on different input sizes (100 random integers, 1000 random integers, 10000, and 100000). Each input size must
be tested a thousand times.

To compile and run the program, make sure you have [Rust and Cargo](https://rustup.rs/) installed.
Once that's done, clone the repository and run the following command from within that directory:
```
cargo run --release
```
This will fetch the program's dependencies (just [rand](https://crates.io/crates/rand) at this point)
and then compile and run an optimized version of the program.
