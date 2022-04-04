use std::ops::{Deref, DerefMut};

// Enum to identify directions
// To be used with a Bd method
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
    // Transposes a board
    // Implemented to enable omnidirectional methods
    fn transp(&self) -> Bd {
        // Initialize empty board with previous score
        let mut transp = Bd {
            board : vec![Vec::with_capacity(self.len()); self.len()],  // Empty board with predefined sizes
            score : self.score, // Copy score over
        };

        // Iteration loop for transposition
        self.iter() // Split board into rows
        .for_each( |row|
            row.iter().zip(0..self.len()) // Split row again into values, adding indexes to each
            .for_each( |(v, tcol)|
                transp[tcol].push(*v) // Rush each value in [row][column] to transp[row]
            )
        );

        transp // Return transposed
    }
}

// Main function, keep it small
fn main() {
    let mut board = Bd::gen(4); // Main board on game unless future implementations of multi-boards


    loop {};

}
