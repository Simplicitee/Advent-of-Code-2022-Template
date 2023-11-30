use std::{io::{Lines, BufReader, BufRead}, fs::File, path::Path};
use solver::*;

mod solver;

// NO NEED TO EDIT THIS FILE (but feel free to do so)

// IMPORTANT: Make sure to put both input datas into their files
// The data for the example solution to the puzzle goes in the example file
// The data for your solution goes into the solving file

const EXAMPLE: &str = "src/inputs/example.txt";
const SOLVING: &str = "src/inputs/solving.txt";

fn main() {
    println!("Test Answers: (do I match the answers to the example from the puzzle?)");
    println!("- Part 1: {}", part_one(read_lines(EXAMPLE)));
    println!("- Part 2: {}", part_two(read_lines(EXAMPLE)));
    println!();
    println!("Your Answers: (put these into the answer after testing your solutions)");
    println!("- Part 1: {}", part_one(read_lines(SOLVING)));
    println!("- Part 2: {}", part_two(read_lines(SOLVING)));
}

fn read_lines<P>(filename: P) -> Lines<BufReader<File>> where P: AsRef<Path> {
    BufReader::new(match File::open(filename) {
        Err(err) => panic!("Error while opening file: {}", err),
        Ok(file) => file,
    }).lines()
}