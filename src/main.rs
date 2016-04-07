use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn check_args() -> (usize, Result<Vec<String>, Vec<String>>) {
    let argv: Vec<_> = env::args().collect();
    let argc = argv.len();

    (argc, match argc {
        1 => Err(argv),
        _ => Ok(argv),
    })
}

fn read_files(argc: usize, argv: Vec<String>) -> Vec<String> {
    let mut buffer = String::new();
    let mut result: Vec<String>;

    for i in 0..argc {
        let mut f = match File::open(argv[i]) {
                        Ok(n) => n,
                        _ => panic!("Could not open file {}", argv[i]),
                    };
        match f.read_to_string(&mut buffer) {
            Ok(n) => n,
            _ => panic!("Could not read file {}", argv[i]),
        };
        result.push(buffer);
    }
    result
}

fn main() {
    let (argc, argv) = match check_args() {
        (n, Ok(m)) => (n, m),
        (_, Err(m)) => { println!("Usage:\t{} puzzle_file [extra_puzzles]", m[0]); return },
    };
    let result = read_files(argc, argv);

    for i in 0..argc {
        println!("File {} => {}", i, result[i]);
    }
}
