use rand::seq::SliceRandom;
use rand::thread_rng;
use rust_1_maze::common::{get_neighbor, Direction, MazeRoom};
use std::cell::RefCell;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Incorrect number of arguments.");
        println!("./generator <output file> <number of rows> <number of columns>");
        return;
    }
}

/* generator.c */
/* ******************************************/
fn get_opposite_dir(dir: Direction) -> Direction {
    // TODO
    dir
}

fn shuffle_directions(directions: &mut Vec<Direction>) -> () {
    // See here - https://stackoverflow.com/questions/26033976/how-do-i-create-a-vec-from-a-range-and-shuffle-it
    // for implementation detail and links to docs.
    directions.shuffle(&mut thread_rng());
}

fn drunken_walk(
    row: usize,
    col: usize,
    num_rows: usize,
    num_cols: usize,
    maze: RefCell<Vec<Vec<MazeRoom>>>, // pass as RefCell so that it can be borrowed both mutably and immutably
) -> () {
    // TODO
    ()
}
