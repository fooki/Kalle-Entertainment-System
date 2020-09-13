use base::center;
use base::SceneUpdate;
use std::rc::Rc;

use ggez::graphics::{Rect, Color};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use ggez::event::{MouseButton};

use ggez::graphics::{draw, Text, DrawParam, Font, TextFragment, Scale};


pub struct TetrisGame {
    text: Text
}

impl TetrisGame {
    pub fn new(ctx: &mut Context) -> TetrisGame {
        let fancy_font = Font::new(ctx, "/boxy.ttf").expect("boom");
        let (x, y) = center(ctx);

        let text = Text::new(
            TextFragment::new("Tetris!!")
                .font(fancy_font)
                .scale(Scale::uniform(40.0))
        );

        TetrisGame {
            text: text
        }
    }
}

impl base::Scene for TetrisGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<SceneUpdate> {
        Ok(SceneUpdate::Nothing)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let base_params = DrawParam::new().color(Color::new(1.0, 1.0, 1.0, 1.0));

        draw(
            ctx,
            &self.text,
            base_params
        )?;

        Ok(())
    }
}
