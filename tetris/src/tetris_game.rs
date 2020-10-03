use base::center;
use base::SceneUpdate;

use crate::board::Board;
use crate::tetris_block::TetrisBlock;

use ggez::mint::Point2;
use ggez::timer::check_update_time;
use ggez::{Context, GameResult};
use ggez::graphics::{self, draw, Color, Rect, Text, DrawParam, Font, TextFragment, Scale};

pub struct TetrisGame {
    board: Board,
    text: Text,
    inverted_speed: u32,
    tick_count: u32,
}

impl TetrisGame {
    pub fn new(ctx: &mut Context) -> TetrisGame {
        let fancy_font = Font::new(ctx, "/boxy.ttf").expect("boom");
        let (_x, _y) = center(ctx);

        let text = Text::new(
            TextFragment::new("Tetris!!")
                .font(fancy_font)
                .scale(Scale::uniform(40.0))
        );

        let mut board = Board::new();
        board.set_current(0,0,TetrisBlock::T);

        TetrisGame {
            text, board, tick_count: 0, inverted_speed: 60
        }
    }
}

impl TetrisGame {
    fn draw_board(&mut self, ctx: &mut Context) -> GameResult<()> {
        let color = [0.1, 0.1, 0.1, 1.0].into();
        let (center_x, center_y) = center(ctx);
        let (x, y) = (center_x-100.0, center_y-200.0);

        let rectangle =
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(
                    0.0,
                    0.0,
                    10.0 * self.block_size(),
                    20.0 * self.block_size()
                ),
                color
            )?;

        draw(ctx, &rectangle, (Point2 { x, y },))?;
        Ok(())
    }

    #[inline]
    fn block_size(&self) -> f32 { 20.0 }

    fn draw_current_figure(&mut self, ctx: &mut Context) -> GameResult<()> {
        let color = [0.8, 0.8, 0.8, 1.0].into();
        let (center_x, center_y) = center(ctx);
        let (base_x, base_y) = (center_x-100.0, center_y-200.0);

        if let Some(current) = self.board.current() {

        let rectangle =
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(
                    0.0,
                    0.0,
                    self.block_size(),
                    self.block_size()
                ),
                color
            )?;

            for (delta_x, delta_y) in current.blocks().iter() {
                let x = (*delta_x as f32 * self.block_size()) + base_x;
                let y = (*delta_y as f32 * self.block_size()) + base_y;
                draw(ctx, &rectangle, (Point2 { x, y },))?;
            }
        }

        Ok(())
    }
}

impl base::Scene for TetrisGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<SceneUpdate> {
        const DESIRED_FPS: u32 = 60;

        while check_update_time(ctx, DESIRED_FPS) {
            self.tick_count += 1;
            if self.tick_count >= self.inverted_speed {
                self.board.tick();
                self.tick_count = 0;
            }
        }
        Ok(SceneUpdate::Nothing)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let base_params = DrawParam::new().color(Color::new(1.0, 1.0, 1.0, 1.0));

        self.draw_board(ctx)?;
        self.draw_current_figure(ctx)?;
        draw(ctx, &self.text, base_params)?;

        Ok(())
    }
}
