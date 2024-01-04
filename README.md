# 1brc
The One Billion Row Challenge - A fun exploration of how quickly 1B rows from a text file can be aggregated with Rust

## Setup
```shell
cargo run --release --bin generate_input -- --size=100
```

## Run
```shell
cargo run --release --bin one-billion-row-challenge -- --path=measurements_1000.txt
```

## Reading
- [] https://docs.rs/rayon/1.5.0/rayon/iter/trait.ParallelIterator.html#method.fold