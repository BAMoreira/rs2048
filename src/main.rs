use std::ops::{Deref, DerefMut};

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

// Struct for valid moves
// To be used with a Bd method
struct Vm {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

// Main board struct type
// All should be designed around this as methods
struct Bd {
    board:Vec<Vec<u32>>, // nested vectors for a two-dimension array
    score: u32, // score inside board for future ease when implementing move method
}

// Deref traits for Bd to call it as matrix
impl Deref for Bd {
    type Target = Vec<Vec<u32>>;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}
impl DerefMut for Bd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.board
    }
}

// Methods and associated functions for Bd
// Pretty much most of the functions will be in here
impl Bd {
    // Generate a new empty board with variable size
    // Both board and scores should be all zero, with spawn added later
    fn gen(a:usize) -> Bd {
        Bd {
            board: vec![vec![0;a];a],
            score: 0,
        }
    }
}

// Main function, keep it small
fn main() {
    let mut board = Bd::gen(4); // Main board on game unless future implementations of multi-boards


}
