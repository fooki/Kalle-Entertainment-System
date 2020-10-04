use crate::tetris_block::TetrisBlock;
use crate::figure_state::FigureState;

type BoardCells = [[TetrisBlock; 10]; 20];

pub struct Board {
    pub current_figure: Option<FigureState>,
    cells: BoardCells,
    generator: fn() -> TetrisBlock,
}

impl Board {
    pub fn new(generator: fn() -> TetrisBlock) -> Self {
        let cells = [[TetrisBlock::Empty; 10]; 20];
        Self { cells, generator, current_figure: None }
    }

    pub fn cells(&self) -> &BoardCells {
        &self.cells
    }

    pub fn tick(&mut self) {
        if let Some(ref mut figure) = self.current_figure {
            figure.translate(0,1);
        } else {
            self.current_figure = Some(FigureState::new(4, 0, (self.generator)()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_cells() -> BoardCells {
        [[TetrisBlock::Empty; 10]; 20]
    }

    fn generator() -> TetrisBlock {
        TetrisBlock::O
    }

    #[test]
    fn test_new_does_not_crash() {
        Board::new(generator);
    }

    #[test]
    fn test_initial_board_returns_empty_cells() {
        let board = Board::new(generator);
        assert_eq!(&empty_cells(), board.cells());
    }

    #[test]
    fn test_tick_does_nothing_without_a_current_figure() {
        let mut board = Board::new(|| TetrisBlock::O);
        board.tick();
    }

    #[test]
    fn test_tick_without_current_adds_a_current_from_strategy() {
        let generator: fn() -> TetrisBlock = || TetrisBlock::O;
        let mut board = Board::new(generator);

        assert!(board.current_figure.is_none());
        board.tick();
        assert_eq!(board.current_figure.unwrap().kind, TetrisBlock::O);
    }
}
