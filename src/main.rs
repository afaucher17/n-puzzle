use std::error::Error;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

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
    for i in 1..argc {
        let path = Path::new(&argv[i]);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("Could not open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };
        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Err(why) => panic!("Could not read {}: {}", display, Error::description(&why)),
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

    for i in 0..argc - 1 {
        println!("File {} => {}", i, result[i]);
    }
}
