use crate::tetris_block::TetrisBlock;
use crate::figure::Figure;

type BoardCells = [[TetrisBlock; 10]; 20];

pub struct Board {
    current_figure: Option<Figure>,
    cells: BoardCells,
}

impl Board {
    pub fn new() -> Self {
        let cells = [[TetrisBlock::Empty; 10]; 20];
        Self { cells, current_figure: None }
    }

    pub fn set_current(&mut self, x: usize, y: usize, kind: TetrisBlock) {
        self.current_figure = Some(Figure::new(x, y, kind));
    }

    pub fn cells(&self) -> &BoardCells {
        &self.cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_cells() -> BoardCells {
        [[TetrisBlock::Empty; 10]; 20]
    }

    #[test]
    fn test_new_does_not_crash() {
        Board::new();
    }

    #[test]
    fn test_set_current_adds_block() {
        let mut board = Board::new();
        board.set_current(5, 0, TetrisBlock::T);
    }

    #[test]
    fn test_initial_board_returns_empty_cells() {
        let board = Board::new();
        assert_eq!(&empty_cells(), board.cells());
    }
}
