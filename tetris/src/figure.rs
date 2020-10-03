use crate::tetris_block::TetrisBlock;

pub struct Figure {
    pub x: usize,
    pub y: usize,
    kind: TetrisBlock
}

impl Figure {
    pub fn new(x: usize, y: usize, kind: TetrisBlock) -> Self {
        Self {x, y, kind }
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.x = (self.x as i32 + x) as usize;
        self.y = (self.y as i32 + y) as usize;
    }

    pub fn kind(&self) -> TetrisBlock {
        self.kind
    }

    pub fn blocks(&self) -> [(usize,usize);4] {
        match self.kind {
            TetrisBlock::I => {
                [(self.x, self.y),
                 (self.x, self.y + 1),
                 (self.x, self.y + 2),
                 (self.x, self.y + 3)]
            },

            TetrisBlock::T => {
                [(self.x + 1, self.y),
                 (self.x, self.y + 1),
                 (self.x + 1, self.y + 1),
                 (self.x + 2, self.y + 1)]
            },

            TetrisBlock::O => {
                [(self.x, self.y),
                 (self.x + 1, self.y),
                 (self.x, self.y + 1),
                 (self.x + 1, self.y + 1)]
            },

            TetrisBlock::L => {
                [(self.x, self.y),
                 (self.x, self.y + 1),
                 (self.x, self.y + 2),
                 (self.x + 1, self.y + 2)]
            },

            TetrisBlock::J => {
                [(self.x + 1, self.y),
                 (self.x + 1, self.y + 1),
                 (self.x + 1, self.y + 2),
                 (self.x, self.y + 2)]
            },

            TetrisBlock::S => {
                [(self.x, self.y + 1),
                 (self.x + 1, self.y + 1),
                 (self.x + 1, self.y),
                 (self.x + 2, self.y)]
            },

            TetrisBlock::Z => {
                [(self.x, self.y),
                 (self.x + 1, self.y),
                 (self.x + 1, self.y + 1),
                 (self.x + 2, self.y + 1)]
            },

            _ => { panic!("Bad block") }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_does_not_crash() {
        let _figure = Figure::new(0, 5, TetrisBlock::L);
    }

    #[test]
    fn test_translate_does_not_crash() {
        let mut figure = Figure::new(3, 5, TetrisBlock::L);
        figure.translate(1, 4);
    }

    #[test]
    fn test_kind_returns_its_block_type() {
        let figure = Figure::new(3, 5, TetrisBlock::O);
        assert_eq!(figure.kind(), TetrisBlock::O)
    }

    #[test]
    #[should_panic]
    fn test_blocks_panics_on_empty_block() {
        Figure::new(0, 0, TetrisBlock::Empty).blocks();
    }

    #[test]
    fn test_blocks_returns_all_positions_for_i_based_on_pos() {
        let figure = Figure::new(0, 0, TetrisBlock::I);
        assert_eq!(figure.blocks(), [(0,0), (0,1), (0,2), (0,3)]);

        let figure = Figure::new(2, 5, TetrisBlock::I);
        assert_eq!(figure.blocks(), [(2,5), (2,6), (2,7), (2,8)]);
    }

    #[test]
    fn test_blocks_returns_all_positions_for_t_based_on_pos() {
        let figure = Figure::new(0, 0, TetrisBlock::T);
        assert_eq!(figure.blocks(), [(1,0), (0,1), (1,1), (2,1)]);

        let figure = Figure::new(3, 3, TetrisBlock::T);
        assert_eq!(figure.blocks(), [(4,3), (3,4), (4,4), (5,4)]);
    }

    #[test]
    fn test_blocks_returns_all_positions_for_o_based_on_pos() {
        let figure = Figure::new(0, 0, TetrisBlock::O);
        assert_eq!(figure.blocks(), [(0,0), (1,0), (0,1), (1,1)]);

        let figure = Figure::new(4, 1, TetrisBlock::O);
        assert_eq!(figure.blocks(), [(4,1), (5,1), (4,2), (5,2)]);
    }

    #[test]
    fn test_blocks_returns_all_positions_for_l_based_on_pos() {
        let figure = Figure::new(0, 0, TetrisBlock::L);
        assert_eq!(figure.blocks(), [(0,0), (0,1), (0,2), (1,2)]);

        let figure = Figure::new(2, 3, TetrisBlock::L);
        assert_eq!(figure.blocks(), [(2,3), (2,4), (2,5), (3,5)]);
    }

    #[test]
    fn test_blocks_returns_all_positions_for_j_based_on_pos() {
        let figure = Figure::new(0, 0, TetrisBlock::J);
        assert_eq!(figure.blocks(), [(1,0), (1,1), (1,2), (0,2)]);

        let figure = Figure::new(1, 1, TetrisBlock::J);
        assert_eq!(figure.blocks(), [(2,1), (2,2), (2,3), (1,3)]);
    }

    #[test]
    fn test_blocks_returns_all_positions_for_s_based_on_pos() {
        let figure = Figure::new(0, 0, TetrisBlock::S);
        assert_eq!(figure.blocks(), [(0,1), (1,1), (1,0), (2,0)]);

        let figure = Figure::new(5, 0, TetrisBlock::S);
        assert_eq!(figure.blocks(), [(5,1), (6,1), (6,0), (7,0)]);
    }

    #[test]
    fn test_blocks_returns_all_positions_for_z_based_on_pos() {
        let figure = Figure::new(0, 0, TetrisBlock::Z);
        assert_eq!(figure.blocks(), [(0,0), (1,0), (1,1), (2,1)]);

        let figure = Figure::new(1, 4, TetrisBlock::Z);
        assert_eq!(figure.blocks(), [(1,4), (2,4), (2,5), (3,5)]);
    }

    #[test]
    fn test_translate_moves_around_the_blocks() {
        let mut figure = Figure::new(0, 0, TetrisBlock::O);
        assert_eq!(figure.blocks()[0], (0, 0));

        figure.translate(3, 2);
        assert_eq!(figure.blocks()[0], (3, 2));

        figure.translate(-1, 2);
        assert_eq!(figure.blocks()[0], (2, 4));
    }
}
