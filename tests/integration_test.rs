use rust_1_maze::common::{initialize_maze, Direction, MazeRoom};
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

#[test]
fn test_write_and_read() {
    let f = CleanupWhenDone::new("test_maze10.txt".to_string());
    // TODO: Run your tests here. You can write to 'test_maze10.txt' and it will be removed once
    // the tests have finished running.
}
