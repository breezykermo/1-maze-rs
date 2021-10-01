use std::collections::HashMap;

/* common.c */
/* ******************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    NORTH = 0b0001,
    SOUTH = 0b0010,
    WEST = 0b0100,
    EAST = 0b1000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderState {
    Uninitialized,
    Opening,
    Wall,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MazeRoom<'a> {
    pub row: usize, // 1 indexed. 0 is an invalid room
    pub col: usize, // 1 indexed. 0 is an invalid room
    pub borders: HashMap<Direction, BorderState>,
    pub next: Option<&'a Self>,
}

fn get_bit_at(input: u16, n: u8) -> BorderState {
    if n < 16 {
        if input & (1 << n) != 0 {
            BorderState::Wall
        } else {
            BorderState::Opening
        }
    } else {
        BorderState::Opening
    }
}

//  Indicates which order bit in a MazeRoom's compressed representation represents a border in
//  a particular direction. This is not arbitrary, but specified in the assignment blurb.
fn get_direction_for_bit_order(order: u8) -> Direction {
    match order {
        3 => Direction::EAST,
        2 => Direction::WEST,
        1 => Direction::SOUTH,
        0 => Direction::NORTH,
        _ => panic!("You can't call this function with any number greater than 3"),
    }
}

fn create_borders() -> HashMap<Direction, BorderState> {
    HashMap::from([
        (Direction::NORTH, BorderState::Uninitialized),
        (Direction::SOUTH, BorderState::Uninitialized),
        (Direction::WEST, BorderState::Uninitialized),
        (Direction::EAST, BorderState::Uninitialized),
    ])
}

impl<'a> MazeRoom<'a> {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            borders: create_borders(),
            next: None,
        }
    }

    pub fn borders_as_char(&self) -> char {
        let as_u16 = self.borders.iter().fold(0x0000, |acc, (dir, is_wall)| {
            let to_add = if *is_wall == BorderState::Wall {
                *dir as u16
            } else {
                0b0000
            };
            acc | to_add
        });
        format!("{:x}", as_u16).chars().next().unwrap()
    }
}

impl<'a> From<(usize, usize, char)> for MazeRoom<'a> {
    fn from((row, col, borders_as_char): (usize, usize, char)) -> Self {
        let as_u16 = u16::from_str_radix(&borders_as_char.to_string(), 16)
            .expect("Could not read maze on disk");
        let mut borders = create_borders();
        for bit_order in 0..4 {
            let bit_order = bit_order as u8;
            let direction = get_direction_for_bit_order(bit_order);
            let border_state = get_bit_at(as_u16, bit_order);
            borders.insert(direction, border_state);
        }
        Self {
            row,
            col,
            borders,
            next: None,
        }
    }
}

pub fn is_in_range(row: usize, col: usize, num_rows: usize, num_cols: usize) -> bool {
    return row > 0 && row < num_rows && col > 0 && col < num_cols;
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

pub fn initialize_maze<'a>(
    num_cols: usize,
    num_rows: usize,
    maze: &mut Vec<Vec<MazeRoom<'a>>>,
) -> () {
    for col in 1..=num_cols {
        for row in 1..=num_rows {
            maze[col - 1][row - 1].row = row;
            maze[col - 1][row - 1].col = col;
        }
    }
}
