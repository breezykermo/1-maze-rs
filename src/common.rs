/* common.c */
/* ******************************************/

pub enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EncodedMazeRoom {}

#[derive(Clone, Debug, PartialEq)]
pub struct MazeRoom {
    // TODO
}

impl MazeRoom {
    pub fn new() -> Self {
        Self {}
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
