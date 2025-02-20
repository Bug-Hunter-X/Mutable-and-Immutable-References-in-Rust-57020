# Mutable and Immutable References in Rust

This repository demonstrates a common error in Rust when working with mutable and immutable references.  It also shows the correct way to handle such situations to avoid common pitfalls and runtime errors.

The code in `bug.rs` showcases a simple example of how seemingly correct code can lead to unexpected behavior due to improper handling of mutable references.

The solution provided in `bugSolution.rs` demonstrates a corrected approach that correctly modifies the value while preserving memory safety.  Pay close attention to the differences and how careful reference management avoids potential compiler errors and runtime panics.