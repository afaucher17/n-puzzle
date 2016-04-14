mod parser;
mod node;
mod astar;
mod heuristic;

use heuristic::Heuristic;

use std::error::Error;
use std::hash::{Hash, Hasher, SipHasher};
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn read_files(filename: &String) -> String {
        let path = Path::new(filename);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("Could not open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };
        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Err(why) => panic!("Could not read {}: {}", display, Error::description(&why)),
            Ok(_) => buffer,
        }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let argv : Vec<String> = env::args().collect();
    match argv.len() {
        1 => { println!("Usage:\t{} puzzle_file [extra_puzzles]", &argv[0]); return },
        _ => (),
    };
    let result: String = read_files(&argv[1]);
    let vec: Vec<String> = result.split("\n")
        .map(|s| s.to_string())
        .collect();
    let start = parser::to_node(parser::remove_comments(vec));
    let goal = node::Goal::new(start.len);
    println!("{}\n{}", start, start.get_score(&goal, Heuristic::Manhattan));
    for neighbour in start.get_neighbour()
    {
        println!("score = {}", neighbour.get_score(&goal, Heuristic::Manhattan));
    }
}
