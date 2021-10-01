use rust_1_maze::common::{initialize_maze, BorderState, Direction, MazeRoom};
use rust_1_maze::{read_encoded_maze_from_file, write_encoded_maze_to_file};
use std::fs;

pub struct CleanupWhenDone {
    pub fname: String,
}

impl Drop for CleanupWhenDone {
    fn drop(&mut self) {
        fs::remove_file("test_maze10.txt").expect("woops");
    }
}

impl CleanupWhenDone {
    pub fn new(fname: String) -> Self {
        Self { fname }
    }

    pub fn name(&self) -> String {
        self.fname.clone()
    }
}

fn whackify_maze(maze: &mut Vec<Vec<MazeRoom>>) {
    maze[0][0]
        .borders
        .insert(Direction::WEST, BorderState::Wall);
    maze[0][0]
        .borders
        .insert(Direction::EAST, BorderState::Wall);
    maze[0][0]
        .borders
        .insert(Direction::NORTH, BorderState::Wall);
    maze[0][0]
        .borders
        .insert(Direction::SOUTH, BorderState::Wall);

    maze[1][1]
        .borders
        .insert(Direction::NORTH, BorderState::Wall);
    maze[2][2]
        .borders
        .insert(Direction::EAST, BorderState::Wall);
    maze[3][3]
        .borders
        .insert(Direction::SOUTH, BorderState::Wall);
    maze[4][4]
        .borders
        .insert(Direction::WEST, BorderState::Wall);
    maze[5][5]
        .borders
        .insert(Direction::NORTH, BorderState::Wall);
    maze[6][6]
        .borders
        .insert(Direction::EAST, BorderState::Wall);
    maze[7][7]
        .borders
        .insert(Direction::SOUTH, BorderState::Wall);
    maze[8][8]
        .borders
        .insert(Direction::WEST, BorderState::Wall);
    maze[9][9]
        .borders
        .insert(Direction::NORTH, BorderState::Wall);
}

fn create_maze_all_open<'a>() -> Vec<Vec<MazeRoom<'a>>> {
    let mut maze = vec![vec![MazeRoom::new(); 10]; 10];
    for row in 0..10 {
        for col in 0..10 {
            maze[col][row]
                .borders
                .insert(Direction::NORTH, BorderState::Opening);
            maze[col][row]
                .borders
                .insert(Direction::SOUTH, BorderState::Opening);
            maze[col][row]
                .borders
                .insert(Direction::EAST, BorderState::Opening);
            maze[col][row]
                .borders
                .insert(Direction::WEST, BorderState::Opening);
        }
    }
    maze
}

#[test]
fn test_write_and_read() {
    let f = CleanupWhenDone::new("test_maze10.txt".to_string());
    let mut maze = create_maze_all_open();
    let mut reloaded_maze = vec![vec![MazeRoom::new(); 10]; 10];
    initialize_maze(10, 10, &mut maze);
    whackify_maze(&mut maze);
    write_encoded_maze_to_file(10, 10, &mut maze, &f.name());
    read_encoded_maze_from_file(10, 10, &mut reloaded_maze, &f.name());

    for (row, row_of_rooms) in reloaded_maze.iter().enumerate() {
        for (col, reloaded_room) in row_of_rooms.iter().enumerate() {
            assert_eq!(*reloaded_room, maze[col][row]);
        }
    }
}
