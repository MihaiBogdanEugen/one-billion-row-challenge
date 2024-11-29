# 1brc
The One Billion Row Challenge - A fun exploration of how quickly 1B rows from a text file can be aggregated with Rust

## Setup
```shell
make generate-input size=1000000000
```

## Run
```shell
make run input-path=measurements_1000000000.txt solution=rayon-fx-hash
```
