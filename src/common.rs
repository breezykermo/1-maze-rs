/* common.c */
/* ******************************************/

pub enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EncodedMazeRoom {
    row: usize,
    col: usize,
    room: u16,
}

impl EncodedMazeRoom {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            room: 0,
        }
    }
}

impl From<(usize, usize, char)> for EncodedMazeRoom {
    fn from((row, col, borders_as_char): (usize, usize, char)) -> Self {
        let as_u16 = u16::from_str_radix(&borders_as_char.to_string(), 16)
            .expect("Could not read maze on disk");
        Self {
            row,
            col,
            room: as_u16,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MazeRoom {
    // TODO
}

impl MazeRoom {
    pub fn new() -> Self {
        Self {}
    }

    pub fn borders_as_char(&self) -> char {
        // TODO: return border representation as a character
        "a".chars().take(1).next().unwrap()
    }
}

pub fn is_in_range(row: usize, col: usize, num_rows: usize, num_cols: usize) -> bool {
    // TODO
    true
}

pub fn get_neighbor<'a>(
    num_rows: usize,
    num_cols: usize,
    maze: &'a Vec<Vec<MazeRoom>>,
    room: &MazeRoom,
    dir: Direction,
) -> Result<&'a MazeRoom, &'static str> {
    // TODO
    Ok(&maze[0][0])
}

pub fn initialize_maze<'a>(num_cols: usize, num_rows: usize, maze: &mut Vec<Vec<MazeRoom>>) -> () {
    // TODO
    ()
}
