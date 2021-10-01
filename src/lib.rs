pub mod common;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::Path;

/* GENERATOR LIB */

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// This function differs from the C implementation in that errors are dealt with via panic, rather
/// than a graceful exit.
/// `num_cols` and `num_rows` are unnecessary in rust, as these values are implicitly stored in the
/// 2d Vec.
pub fn write_encoded_maze_to_file(
    _num_rows: u8,
    _num_cols: u8,
    encoded_maze: &mut Vec<Vec<common::MazeRoom>>,
    file_name: &str,
) -> () {
    let mut file_buffer = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
        .expect("Error opening file.");

    for row_of_rooms in encoded_maze.iter() {
        let row: String = row_of_rooms.iter().map(|r| r.borders_as_char()).collect();
        // println!("Writing row: {}", row);
        file_buffer
            .write_all(row.as_bytes())
            .expect("Writing row of rooms failed.");
        file_buffer
            .write_all(NEWLINE_AS_STR.as_bytes())
            .expect("Writing newline failed.");
    }
}

/* SOLVER LIB */

const NEWLINE_AS_STR: &str = "\n";

pub fn read_encoded_maze_from_file(
    _: usize, // num_rows: passed implicitly in encoded_maze dimensionss
    _: usize, // num_cols: passed implicitly in encoded_maze dimensionss
    encoded_maze: &mut Vec<Vec<common::EncodedMazeRoom>>,
    file_name: &str,
) -> () {
    let mut y_count = 1;
    let mut x_count = 1;
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            let line = line.unwrap();
            for c in line.chars() {
                encoded_maze[x_count - 1][y_count - 1] =
                    common::EncodedMazeRoom::from((x_count, y_count, c));
                x_count += 1;
            }
            y_count += 1;
            x_count = 1;
        }
    }
}

/* TESTS */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initialize_maze() {}
}
