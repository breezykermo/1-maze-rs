use rust_1_maze::common::MazeRoom;
use rust_1_maze::read_encoded_maze_from_file;
use std::fs::File;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Incorrect number of arguments.");
        println!("./generator <output file> <number of rows> <number of columns>");
        return;
    }

    let file_name: &str = &args[1];
    let num_rows: usize = args[2].parse().unwrap();
    let num_cols: usize = args[3].parse().unwrap();

    let mut encoded_maze = vec![vec![MazeRoom::new(); num_cols]; num_rows];
    read_encoded_maze_from_file(num_rows, num_cols, &mut encoded_maze, file_name);

    println!("{:?}", encoded_maze);
}

/* solver.c */
/* ******************************************/
fn create_room_connections(maze_room: &MazeRoom, hex: u8) -> () {}

fn dfs(
    row: u8,
    col: u8,
    goal_row: u8,
    goal_col: u8,
    num_rows: u8,
    num_cols: u8,
    maze: Vec<Vec<MazeRoom>>,
    file: &File,
) -> u8 {
    return 1;
}

fn decode_maze(
    num_rows: u8,
    num_cols: u8,
    maze: Vec<Vec<MazeRoom>>,
    encoded_maze: Vec<Vec<u8>>,
) -> () {
}

fn print_pruned_path(room: &MazeRoom, file: &File) -> u8 {
    return 1;
}
