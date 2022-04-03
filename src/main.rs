use std::ops::{Deref, DerefMut};

struct Bd {
    board:Vec<Vec<u32>>,
    score: u32,
}

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

impl Bd {
    fn gen(a:usize) -> Bd {
        Bd {
            board: vec![vec![0;a];a],
            score: 0,
        }
    }
}

fn main() {
    let mut board = Bd::gen(4);
    board.score = 1;

}
