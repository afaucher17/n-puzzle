use std::error::Error;
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
    let mut result = Vec::new();

    for i in 0..argc {
        let mut file = match File::open(argv[i]) {
            Err(why) => panic!("Could not open {}: {}", argv[i], Error::description(&why)),
            Ok(file) => file,
        };

        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Err(why) => panic!("Could not read {}: {}", argv[i], Error::description(&why)),
            Ok(_) => result.push(buffer),
        };
    };
    result
}

fn main() {
    let (argc, argv) = match check_args() {
        (_, Err(m)) => { println!("Usage:\t{} puzzle_file [extra_puzzles]", m[0]); return },
        (n, Ok(m)) => (n, m),
    };
    let result = read_files(argc, argv);

    for i in 0..argc {
        println!("File {} => {}", i, result[i]);
    }
}
