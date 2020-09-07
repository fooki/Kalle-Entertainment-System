use base::center;

use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::conf::{self, FullscreenType};
use ggez::ContextBuilder;
use ggez::event::{self};
use ggez::graphics::{draw, Text, DrawParam, Font, TextFragment, Scale};
use ggez::event::MouseButton;

pub struct MainMenu {
    text: Text,
    half_text_width: f32,
    half_text_height: f32,
}

impl MainMenu {
    pub fn new(ctx: &mut Context) -> MainMenu {
        let fancy_font = Font::new(ctx, "/boxy.ttf").expect("boom");
        let text = Text::new(
            TextFragment::new("Hey Emelie!")
                .font(fancy_font)
                .scale(Scale::uniform(40.0))
        );
        let half_text_width = text.width(ctx) as f32 / 2.0;
        let half_text_height = text.height(ctx) as f32 / 2.0;
        MainMenu { text, half_text_width, half_text_height }
    }
}

impl base::Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (x, y) = center(ctx);
        draw(
            ctx,
            &self.text,
            DrawParam::new()
                .dest(Point2 {
                    x: (x - self.half_text_width),
                    y: (y - self.half_text_height)
                })
        )?;
        Ok(())
    }
}
