use rand::seq::SliceRandom;
use rand::thread_rng;
use rust_1_maze::common::{get_neighbor, BorderState, Direction, MazeRoom};
use std::cell::RefCell;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Incorrect number of arguments.");
        println!("./generator <output file> <number of rows> <number of columns>");
        return;
    }

    let file_name: String = args[1];
    let num_rows: u8 = u8::from(args[2]);
    let num_cols: u8 = args[3];
}

/* generator.c */
/* ******************************************/
fn get_opposite_dir(dir: Direction) -> Direction {
    match dir {
        Direction::WEST => Direction::EAST,
        Direction::EAST => Direction::WEST,
        Direction::NORTH => Direction::SOUTH,
        Direction::SOUTH => Direction::NORTH,
    }
}

fn shuffle_directions(directions: &mut Vec<Direction>) -> () {
    // See here - https://stackoverflow.com/questions/26033976/how-do-i-create-a-vec-from-a-range-and-shuffle-it
    // for implementation detail and links to docs.
    directions.shuffle(&mut thread_rng());
}

fn get_row_offset(dir: &Direction) -> i8 {
    match *dir {
        Direction::NORTH => -1,
        Direction::SOUTH => 1,
        _ => 0,
    }
}

fn get_col_offset(dir: &Direction) -> i8 {
    match *dir {
        Direction::WEST => -1,
        Direction::EAST => 1,
        _ => 0,
    }
}

fn drunken_walk(
    row: usize,
    col: usize,
    num_rows: usize,
    num_cols: usize,
    maze: RefCell<Vec<Vec<MazeRoom>>>, // pass as RefCell so that it can be borrowed both mutably and immutably
) -> () {
    // // TODO
    let mut visited = vec![vec![false; num_rows]; num_cols];
    visited[col][row] = true;
    let mut all_directions = Vec::from([
        Direction::NORTH,
        Direction::SOUTH,
        Direction::WEST,
        Direction::EAST,
    ]);
    shuffle_directions(&mut all_directions);
    for &dir in all_directions.iter() {
        let maze_immut = maze.borrow();
        let potential_neigbour =
            get_neighbor(num_rows, num_cols, &maze_immut, &maze_immut[col][row], dir);

        let current_room = &mut maze.borrow_mut()[col][row];
        match potential_neigbour {
            Err(_) => {
                let borders = &mut current_room.borders;
                (*borders).insert(dir, BorderState::Wall);
            }
            Ok(neighbour) => {
                match visited[neighbour.col][neighbour.row] {
                    false => {
                        drunken_walk(
                            neighbour.row,
                            neighbour.col,
                            num_rows,
                            num_cols,
                            maze.clone(), // NOTE: this only clones the refcell, i hope
                        );
                    }
                    true => {
                        let opposite_dir = get_opposite_dir(dir);
                        let opposite_wall = *current_room.borders.get(&opposite_dir).unwrap();
                        let borders = &mut current_room.borders;
                        match opposite_wall {
                            BorderState::Uninitialized => {
                                (*borders).insert(dir, BorderState::Wall);
                            }
                            opp_dir => {
                                (*borders).insert(dir, opp_dir);
                            }
                        }
                    }
                }
            }
        }
    }
}
