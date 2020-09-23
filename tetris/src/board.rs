#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TetrisBlock {
    Empty, I, T, O, L, J, S, Z
}

pub struct Board {
    current_type: Option<TetrisBlock>,
    current_position: Option<(usize, usize)>,
    cells: [[TetrisBlock; 10]; 20],
}

impl Board {
    pub fn new() -> Self {
        let cells = [[TetrisBlock::Empty; 10]; 20];
        Self { cells, current_position: None, current_type: None }
    }

    pub fn set_current(&mut self, x: usize, y: usize, block_type: TetrisBlock) {
        self.current_type = Some(block_type);
        self.current_position = Some((x, y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_does_not_crash() {
        Board::new();
    }

    #[test]
    fn test_set_current_adds_block() {
        let mut board = Board::new();
        board.set_current(5, 0, TetrisBlock::T);
    }
}
