src/ contains all the benchmarking programs.
rust_hangman/ contains the code for the Rust implementation of Hangman.
python_hangman/ contains the code for the Python implementation of Hangman.

To run benchmarks for Python, use /usr/bin/time -v python3 {insert script here} {insert parameters here}.
To run benchmarks for Rust, navigate into the cargo folder within each algorithm, then run cargo build for the debug version, and cargo build --release for the release/optimized version.
Then, you can navigate into target/debug/ or target/release/ and run /usr/bin/time -v ./{insert binary here} {insert parameters here}.

The parameters for each benchmark can be found in results.csv.
