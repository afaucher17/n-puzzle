# N-Puzzle

[![travis-badge][]][travis]
[travis-badge]: https://travis-ci.org/adjivas/n-puzzle.svg?branch=master&style=flat-square
[travis]: https://travis-ci.org/adjivas/n-puzzle

### Usage
```text
Npuzzle

Taquin (N-Puzzle) is a sliding puzzle that consists of a frame of numbered square tiles in random order with one tile missing.

USAGE:
    pasteur [FLAGS] [OPTIONS] --file <file>...

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>           Defines the input.
    -h, --heuristic <heuristic>    Defines the heuristic. [values: manhattan]
```

How to run:
```shell
cargo run -- --file <(python npuzzle-gen.py -s 4) <(python npuzzle-gen.py -s 5)
```
