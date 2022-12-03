use std::{io::{Lines, BufReader, BufRead}, fs::File, path::Path};
use lib::*;

mod lib;

// NO NEED TO EDIT THIS FILE (but feel free to do so)

// Make sure to put both input datas into their files!

fn main() {
    println!("Test Answers: (do I match the example from the puzzle?)");
    println!("- Part 1: {}", part_one(read_lines("src/inputs/example.txt")));
    println!("- Part 2: {}", part_two(read_lines("src/inputs/example.txt")));
    println!();

    println!("Answers:");
    println!("- Part 1: {}", part_one(read_lines("src/inputs/solving.txt")));
    println!("- Part 2: {}", part_two(read_lines("src/inputs/solving.txt")));
}

fn read_lines<P>(filename: P) -> Lines<BufReader<File>>
where P: AsRef<Path>, {
    let file = match File::open(filename) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };
    BufReader::new(file).lines()
}