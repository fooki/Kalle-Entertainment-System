use base::center;
use std::rc::Rc;

use ggez::graphics::{Rect, Color};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use ggez::event::{MouseButton, quit};

use ggez::graphics::{draw, Text, DrawParam, Font, TextFragment, Scale};


pub struct MainMenu {
    cursor_state: CursorState,
    play_btn: Rc<Button>,
    quit_btn: Rc<Button>
}

pub struct Button {
    id: i64,
    text: Text,
    half_text_width: f32,
    half_text_height: f32,
    x: f32,
    y: f32
}

impl PartialEq for Button {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Button {
    fn new(ctx: &mut Context, text: Text, x: f32, y: f32) -> Self {
        let half_text_width = text.width(ctx) as f32 / 2.0;
        let half_text_height = text.height(ctx) as f32 / 2.0;

        let id = base::next_id();

        Self { id, text, half_text_width, half_text_height, x, y }
    }

    fn draw(&self, ctx: &mut Context, params: &DrawParam) -> GameResult<()> {
        draw(
            ctx,
            &self.text,
            params.dest(Point2 {
                x: (self.x - self.half_text_width),
                y: (self.y - self.half_text_height)
            })
        )
    }

    fn contains(&self, p: Point2<f32>) -> bool {
        let rect = Rect::new(
            self.x - self.half_text_width,
            self.y - self.half_text_height,
            self.half_text_width * 2.0,
            self.half_text_height * 2.0,
        );
        rect.contains(p)
    }
}

impl MainMenu {
    pub fn new(ctx: &mut Context) -> MainMenu {
        let fancy_font = Font::new(ctx, "/boxy.ttf").expect("boom");
        let (x, y) = center(ctx);

        let text = Text::new(
            TextFragment::new("Play")
                .font(fancy_font)
                .scale(Scale::uniform(40.0))
        );
        let play_btn = Rc::new(Button::new(ctx, text, x, y));


        let text = Text::new(
            TextFragment::new("Quit")
                .font(fancy_font)
                .scale(Scale::uniform(40.0))
        );
        let quit_btn = Rc::new(Button::new(ctx, text, x, y + 100.0));

        MainMenu {
            cursor_state: CursorState::Idle,
            play_btn,
            quit_btn
        }
    }
}

#[derive(PartialEq)]
enum CursorState {
    Idle,
    Hovering(Rc<Button>),
    MouseDown(Rc<Button>)
}

impl base::Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let CursorState::MouseDown(btn) = &self.cursor_state {
            if btn.id == self.quit_btn.id {
                quit(ctx);
            }
        }

        scene_manager.push(Scene:new())
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let base_params = DrawParam::new().color(Color::new(1.0, 1.0, 1.0, 1.0));

        let play_hover =
            self.cursor_state == CursorState::Hovering(self.play_btn.clone());
        let quit_hover =
            self.cursor_state == CursorState::Hovering(self.quit_btn.clone());

        let play_draw_params = if play_hover {
            base_params.color(Color::new(1.0, 0.0, 0.0, 1.0))
        } else {
            base_params
        };

        let quit_draw_params = if quit_hover {
            base_params.color(Color::new(1.0, 0.0, 0.0, 1.0))
        } else {
            base_params
        };

        self.play_btn.draw(ctx, &play_draw_params)?;
        self.quit_btn.draw(ctx, &quit_draw_params)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _btn: MouseButton, x: f32, y: f32) {
        let point = Point2 { x, y };

        if self.play_btn.contains(point) {
            self.cursor_state = CursorState::MouseDown(self.play_btn.clone());
        } else if self.quit_btn.contains(point) {
            self.cursor_state = CursorState::MouseDown(self.quit_btn.clone());
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        let point = Point2 { x, y };

        if self.play_btn.contains(point) {
            self.cursor_state = CursorState::Hovering(self.play_btn.clone());
        } else if self.quit_btn.contains(point) {
            self.cursor_state = CursorState::Hovering(self.quit_btn.clone());
        } else {
            self.cursor_state = CursorState::Idle;
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _btn: MouseButton, _x: f32, _y: f32) {
        self.cursor_state = CursorState::Idle;
    }
}
