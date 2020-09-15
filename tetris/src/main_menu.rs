use base::center;
use base::SceneUpdate;
use std::rc::Rc;

use ggez::graphics::{Rect, Color};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use ggez::event::{MouseButton, Button as Mutton, GamepadId, Axis};

use ggez::graphics::{draw, Text, DrawParam, Font, TextFragment, Scale};

use crate::tetris_game::TetrisGame;
use crate::button_group::ButtonGroup;

pub struct MainMenu<'a> {
    buttons: ButtonGroup<'a>,
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
}

impl<'a> MainMenu<'a> {
    pub fn new(ctx: &mut Context) -> MainMenu<'a> {
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


        let buttons = ButtonGroup::new(&["Play", "Quit"]);
        MainMenu {
            buttons,
            play_btn,
            quit_btn
        }
    }
}

impl<'a> base::Scene for MainMenu<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<SceneUpdate> {
        Ok(SceneUpdate::Nothing)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let base_params = DrawParam::new().color(Color::new(1.0, 1.0, 1.0, 1.0));
        self.play_btn.draw(ctx, &base_params)?;
        self.quit_btn.draw(ctx, &base_params)?;

        Ok(())
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, _value: f32, _id: GamepadId) {

    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Mutton, _id: GamepadId) {

    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: Mutton, _id: GamepadId) {

    }
}
