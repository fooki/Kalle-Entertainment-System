use base::center;
use base::SceneUpdate;

use crate::board::Board;

use ggez::mint::Point2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, draw, Color, Rect, Text, DrawParam, Font, TextFragment, Scale};

pub struct TetrisGame {
    board: Board,
    text: Text,
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

        let board = Board::new();

        TetrisGame {
            text, board
        }
    }
}

impl TetrisGame {
    fn draw_board(&mut self, ctx: &mut Context) -> GameResult<()> {
        let color = [0.1, 0.1, 0.1, 1.0].into();
        let (x, y) = center(ctx);
        let rectangle =
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0.0, 0.0, 200.0, 400.0),
                color
            )?;
        draw(ctx, &rectangle, (Point2 { x: x-100.0, y: y-200.0 },))?;
        Ok(())
    }
}

impl base::Scene for TetrisGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<SceneUpdate> {
        Ok(SceneUpdate::Nothing)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let base_params = DrawParam::new().color(Color::new(1.0, 1.0, 1.0, 1.0));

        self.draw_board(ctx)?;
        draw(ctx, &self.text, base_params)?;

        Ok(())
    }
}
