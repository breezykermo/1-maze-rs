/* common.c */
/* ******************************************/

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
) -> Result<&'a MazeRoom<'a>, &'static str> {
    let (new_col, new_row) = match dir {
        Direction::NORTH => (room.col, room.row - 1),
        Direction::SOUTH => (room.col, room.row + 1),
        Direction::EAST => (room.col + 1, room.row),
        Direction::WEST => (room.col - 1, room.row),
    };
    if !is_in_range(new_row, new_col, num_rows, num_cols) {
        return Err("Neighbour room is not in the maze.");
    }
    Ok(&maze[new_col - 1][new_row - 1])
}

pub fn initialize_maze<'a>(num_cols: usize, num_rows: usize, maze: &mut Vec<Vec<MazeRoom>>) -> () {
    // TODO
    ()
}
