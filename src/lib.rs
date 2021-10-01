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

    // file automatically closes when out of scope
}

/* SOLVER LIB */

const NEWLINE_AS_STR: &str = "\n";

pub fn read_encoded_maze_from_file(
    _: usize, // num_rows: passed implicitly in encoded_maze dimensionss
    _: usize, // num_cols: passed implicitly in encoded_maze dimensionss
    encoded_maze: &mut Vec<Vec<common::MazeRoom>>,
    file_name: &str,
) -> () {
    let mut y_count = 1;
    let mut x_count = 1;
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            let line = line.unwrap();
            for c in line.chars() {
                encoded_maze[x_count - 1][y_count - 1] =
                    common::MazeRoom::from((x_count, y_count, c));
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
    use std::collections::HashMap;
    #[test]
    fn test_initialize_maze() {
        let mut maze_1 = vec![vec![common::MazeRoom::new(); 1]; 1];
        common::initialize_maze(1, 1, &mut maze_1);
        assert_eq!(maze_1.len(), 1);
        assert_eq!(maze_1[0].len(), 1);
        assert_eq!(maze_1[0][0].row, 1);
        assert_eq!(maze_1[0][0].col, 1);
        let borders = &maze_1[0][0].borders;
        assert_eq!(
            *(borders.get(&common::Direction::NORTH)).unwrap(),
            common::BorderState::Uninitialized
        );
        assert_eq!(
            *(borders.get(&common::Direction::SOUTH)).unwrap(),
            common::BorderState::Uninitialized
        );
        assert_eq!(
            *(borders.get(&common::Direction::WEST)).unwrap(),
            common::BorderState::Uninitialized
        );
        assert_eq!(
            *(borders.get(&common::Direction::EAST)).unwrap(),
            common::BorderState::Uninitialized
        );

        let mut maze_10 = vec![vec![common::MazeRoom::new(); 10]; 10];
        common::initialize_maze(10, 10, &mut maze_10);
        assert_eq!(maze_10.len(), 10);
        for row in maze_10.iter() {
            assert_eq!(row.len(), 10);
        }

        let mut maze_5_10 = vec![vec![common::MazeRoom::new(); 10]; 5];
        common::initialize_maze(5, 10, &mut maze_5_10);
        assert_eq!(maze_5_10.len(), 5);
        for row in maze_5_10.iter() {
            assert_eq!(row.len(), 10);
        }
    }

    #[test]
    fn test_get_neighbour_happy_path() {
        let mut maze10 = vec![vec![common::MazeRoom::new(); 10]; 10];
        common::initialize_maze(10, 10, &mut maze10);

        // modfiy room so we can check the west value
        maze10[5][5]
            .borders
            .insert(common::Direction::WEST, common::BorderState::Wall);

        let room_going_north =
            common::get_neighbor(10, 10, &maze10, &maze10[5][6], common::Direction::NORTH)
                .expect("Get neighbour returned an error.");
        assert_eq!(
            *room_going_north
                .borders
                .get(&common::Direction::WEST)
                .unwrap(),
            *maze10[5][5].borders.get(&common::Direction::WEST).unwrap(),
        );

        let room_going_south =
            common::get_neighbor(10, 10, &maze10, &maze10[5][4], common::Direction::SOUTH)
                .expect("Get neighbour returned an error.");
        assert_eq!(
            *room_going_south
                .borders
                .get(&common::Direction::WEST)
                .unwrap(),
            *maze10[5][5].borders.get(&common::Direction::WEST).unwrap(),
        );

        let room_going_east =
            common::get_neighbor(10, 10, &maze10, &maze10[4][5], common::Direction::EAST)
                .expect("Get neighbour returned an error.");
        assert_eq!(
            *room_going_east
                .borders
                .get(&common::Direction::WEST)
                .unwrap(),
            *maze10[5][5].borders.get(&common::Direction::WEST).unwrap(),
        );

        let room_going_west =
            common::get_neighbor(10, 10, &maze10, &maze10[6][5], common::Direction::WEST)
                .expect("Get neighbour returned an error.");
        assert_eq!(
            *room_going_west
                .borders
                .get(&common::Direction::WEST)
                .unwrap(),
            *maze10[5][5].borders.get(&common::Direction::WEST).unwrap(),
        );
    }

    #[test]
    #[should_panic(expected = "Neighbour room is not in the maze.")]
    fn test_get_neighbour_north_error() {
        let mut maze10 = vec![vec![common::MazeRoom::new(); 10]; 10];
        common::initialize_maze(10, 10, &mut maze10);

        let _ =
            common::get_neighbor(10, 10, &maze10, &maze10[4][0], common::Direction::NORTH).unwrap();
    }

    #[test]
    #[should_panic(expected = "Neighbour room is not in the maze.")]
    fn test_get_neighbour_south_error() {
        let mut maze10 = vec![vec![common::MazeRoom::new(); 10]; 10];
        common::initialize_maze(10, 10, &mut maze10);

        let _ =
            common::get_neighbor(10, 10, &maze10, &maze10[4][9], common::Direction::SOUTH).unwrap();
    }

    #[test]
    #[should_panic(expected = "Neighbour room is not in the maze.")]
    fn test_get_neighbour_east_error() {
        let mut maze10 = vec![vec![common::MazeRoom::new(); 10]; 10];
        common::initialize_maze(10, 10, &mut maze10);

        let _ =
            common::get_neighbor(10, 10, &maze10, &maze10[9][2], common::Direction::EAST).unwrap();
    }

    #[test]
    #[should_panic(expected = "Neighbour room is not in the maze.")]
    fn test_get_neighbour_west_error() {
        let mut maze10 = vec![vec![common::MazeRoom::new(); 10]; 10];
        common::initialize_maze(10, 10, &mut maze10);

        let _ =
            common::get_neighbor(10, 10, &maze10, &maze10[9][9], common::Direction::EAST).unwrap();
    }

    #[test]
    fn test_as_u8() {
        let mut room = common::MazeRoom::new();
        let orig_room = room.clone();
        room.borders
            .insert(common::Direction::EAST, common::BorderState::Wall);
        assert_eq!(
            room.borders_as_char(),
            (format!("{:x}", 0b1000).chars().next().unwrap())
        );
        room.borders = orig_room.borders.clone();
        room.borders
            .insert(common::Direction::WEST, common::BorderState::Wall);
        assert_eq!(
            room.borders_as_char(),
            (format!("{:x}", 0b0100).chars().next().unwrap())
        );
        room.borders = orig_room.borders.clone();
        room.borders
            .insert(common::Direction::SOUTH, common::BorderState::Wall);
        assert_eq!(
            room.borders_as_char(),
            (format!("{:x}", 0b0010).chars().next().unwrap())
        );
        room.borders = orig_room.borders.clone();
        room.borders
            .insert(common::Direction::NORTH, common::BorderState::Wall);
        assert_eq!(
            room.borders_as_char(),
            (format!("{:x}", 0b0001).chars().next().unwrap())
        );

        room.borders = orig_room.borders.clone();
        room.borders
            .insert(common::Direction::NORTH, common::BorderState::Wall);
        room.borders
            .insert(common::Direction::SOUTH, common::BorderState::Wall);
        assert_eq!(
            room.borders_as_char(),
            (format!("{:x}", 0b0011).chars().next().unwrap())
        );

        room.borders = orig_room.borders.clone();
        room.borders
            .insert(common::Direction::EAST, common::BorderState::Wall);
        room.borders
            .insert(common::Direction::WEST, common::BorderState::Wall);
        room.borders
            .insert(common::Direction::NORTH, common::BorderState::Wall);
        assert_eq!(
            room.borders_as_char(),
            (format!("{:x}", 0b1101).chars().next().unwrap())
        );
    }

    #[test]
    fn compress_and_decompress() {
        let mr = common::MazeRoom {
            row: 0,
            col: 0,
            borders: HashMap::from([
                (common::Direction::EAST, common::BorderState::Wall),
                (common::Direction::WEST, common::BorderState::Wall),
                (common::Direction::NORTH, common::BorderState::Opening),
                (common::Direction::SOUTH, common::BorderState::Opening),
            ]),
            next: None,
        };
        let room_as_char = mr.borders_as_char();
        assert_eq!(mr, common::MazeRoom::from((0, 0, room_as_char)));
    }
}
