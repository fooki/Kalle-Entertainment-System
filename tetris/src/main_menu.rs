use base::center;

use ggez::graphics::{Rect};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::conf::{self, FullscreenType};
use ggez::ContextBuilder;
use ggez::event::{self};
use ggez::graphics::{draw, Text, DrawParam, Font, TextFragment, Scale};
use ggez::event::MouseButton;

pub struct MainMenu {
    play_btn: Button,
    quit_btn: Button
}

pub struct Button {
    text: Text,
    half_text_width: f32,
    half_text_height: f32,
    x: f32,
    y: f32
}

impl Button {
    fn new(ctx: &mut Context, text: Text, x: f32, y: f32) -> Self {
        let half_text_width = text.width(ctx) as f32 / 2.0;
        let half_text_height = text.height(ctx) as f32 / 2.0;

        Self { text, half_text_width, half_text_height, x, y }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        draw(
            ctx,
            &self.text,
            DrawParam::new()
                .dest(Point2 {
                    x: (self.x - self.half_text_width),
                    y: (self.y - self.half_text_height)
                })
        )
    }

    fn contains(&self, p: Point2<f32>) -> bool {
        let rect = Rect::new(
            (self.x - self.half_text_width),
            (self.y - self.half_text_height),
            self.half_text_width * 2.0,
            self.half_text_height * 2.0,
        );
        rect.contains(p)
    }
}

impl MainMenu {
    pub fn new(ctx: &mut Context) -> MainMenu {
        let fancy_font = Font::new(ctx, "/boxy.ttf").expect("boom");

        let text = Text::new(
            TextFragment::new("Play")
                .font(fancy_font)
                .scale(Scale::uniform(40.0))
        );

        let (x, y) = center(ctx);
        let play_btn = Button::new(ctx, text, x, y);


        let text = Text::new(
            TextFragment::new("Quit")
                .font(fancy_font)
                .scale(Scale::uniform(40.0))
        );
        let (x, y) = center(ctx);
        let quit_btn = Button::new(ctx, text, x, y + 100.0);
        MainMenu { play_btn, quit_btn }
    }
}

impl base::Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.play_btn.draw(ctx)?;
        self.quit_btn.draw(ctx)?;

        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {

    }
}
