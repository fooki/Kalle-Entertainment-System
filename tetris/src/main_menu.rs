use base::center;
use base::SceneUpdate;
use std::rc::Rc;

use ggez::mint::Point2;
use ggez::{Context, GameResult};
use ggez::input::keyboard::{self,KeyMods, KeyCode};
use ggez::graphics::{draw, Color, Text, DrawParam, Font, TextFragment, Scale};

use crate::tetris_game::TetrisGame;
use crate::button_group::ButtonGroup;

pub struct MainMenu<'a> {
    buttons: ButtonGroup<'a>,
    play_btn: Rc<Button>,
    quit_btn: Rc<Button>,
    button_a: bool,
    button_b: bool,

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

        let button_a = false;
        let button_b = false;

        let buttons = ButtonGroup::new(&["Play", "Quit"]);
        MainMenu {
            buttons,
            play_btn,
            quit_btn,
            button_a,
            button_b,
        }
    }
}

impl<'a> base::Scene for MainMenu<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<SceneUpdate> {
        if self.button_a {
            if self.buttons.current() == "Play" {
                return Ok(SceneUpdate::Change(
                    Box::new(TetrisGame::new(ctx)))
                )
            }

            if self.buttons.current() == "Quit" {
                return Ok(SceneUpdate::Quit)
            }
        }

        self.button_a = false;
        self.button_b = false;
        Ok(SceneUpdate::Nothing)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let base_params = DrawParam::new().color(Color::new(1.0, 1.0, 1.0, 1.0));

        let play_params = if self.buttons.current() == "Play" {
            base_params.color(Color::new(1.0, 0.0, 0.0, 1.0))
        } else {
            base_params
        };

        let quit_params = if self.buttons.current() == "Quit" {
            base_params.color(Color::new(1.0, 0.0, 0.0, 1.0))
        } else {
            base_params
        };

        self.play_btn.draw(ctx, &play_params)?;
        self.quit_btn.draw(ctx, &quit_params)?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, _keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.buttons.up()
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.buttons.down()
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.button_a = true
        }

        if keyboard::is_key_pressed(ctx, KeyCode::B) {
            self.button_b = true
        }
    }
}
