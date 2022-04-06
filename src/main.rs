/**************************************************************************************************
*                                                                                                 *
*    Copyright 2022 Bernardo de Azevedo Moreira                                                   *
*                                                                                                 *
*    This file is part of BAMoreira/rs2048.                                                       *
*                                                                                                 *
*    rs2048 is free software: you can redistribute it and/or modify it under the terms of the GNU *
*    General Public License as published by the Free Software Foundation, either version 3 of the *
*    License, or (at your option) any later version.                                              *
*                                                                                                 *
*    rs2048 is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without  *
*    even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the    *
*    GNU General Public License for more details.                                                 *
*                                                                                                 *
*    You should have received a copy of the GNU General Public License along with this file.      *
*    If not, see http://www.gnu.org/licenses/.                                                    *
*                                                                                                 *
**************************************************************************************************/

use std::ops::{Deref, DerefMut};
use crate::Dir::*; // For ease of calling directions

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
#[derive(Debug)] // For debug purposes, remove later
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

// Deref traits for Bd to call it as matrix and use Vec methods
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
    // Generates the full list of valid moves, calling another method Bd::valid() to do so
    fn valid_moves(&self) -> Vm {
        Vm {
            left: self.valid(Left),
            right: self.valid(Right),
            up: self.valid(Up),
            down: self.valid(Down),
        }
    }
    // Checks whether a move in given direction is valid
    // For right, it flips the board horizontally to read in reverse
    // For up and down, it transposes the board to read as left and right
    fn valid(&self, dir: Dir) -> bool {
        // First match decides whether there's a need to transpose the board or not
        // And then separates the board, transposed or not, into its constituent rows
        let mut bd = match dir {
            Left|Right => self.board.clone().into_iter(), // Need to directly call board and clone
            Up|Down => self.transp().board.clone().into_iter(), // Because transp() is temporary
        };
        bd.find( |row| { // Iterating find on separated rows
            let mut nrow = row.clone(); // Bind cloned row
            match dir {
                Right|Down => nrow.reverse(), // To be able to flip them around without massive
                _ => (),                      // Nestled Iter methods
            };
            let mut irow = nrow.windows(2); // Split the row into sequential and overlapping pairs
            irow.find(|pair| { // Iterating find on each pair, Looking for a specific pattern:
                match pair {   // We match the pair here to handle refutability
                    [prev,cur] =>
                        (*cur != 0) &&     // Current block must not be empty, and
                        ((prev == cur) ||  // The previous block must be either equal to the current,
                        (*prev == 0)),     // Or completely empty
                    _ => false,
                }
            }).is_some() // The return of the inner find feeds into the outer find
        }).is_some() // The find iterations succeed if at any time there is a movable pattern found
    }
    fn _apply_move(self, _dir: Dir) -> Bd {
        self
    }
}

// Main function, keep it small
fn main() {
    let mut board = Bd::gen(4); // Main board on game unless future implementations of multi-boards

    board[3][3] = 44;
    board[3][2] = 33;
    board[3][1] = 11;
    board[3][0] = 5;
    // Debug testing for confirmation
    for r in board.iter() { println!("{r:?}"); }

    let a = board.valid_moves();

    println!("{:?}", a);

    loop {};

}
