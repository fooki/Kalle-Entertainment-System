use ggez::{graphics, Context, GameResult};
use ggez::timer::fps;
use ggez::graphics::{draw, Text, DrawParam};
use ggez::event::{quit, EventHandler, MouseButton, Button, GamepadId, Axis, KeyCode};
use ggez::input::keyboard::KeyMods;


use crate::scene::*;

pub struct Game {
    draw_fps: bool,
    scene: Box<dyn Scene>
}

impl Game {
    pub fn new(_ctx: &mut Context, draw_fps: bool, scene: Box<dyn Scene>) -> Game {
        Game { draw_fps, scene }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let result = self.scene.update(ctx)?;
        match result {
            SceneUpdate::Quit => quit(ctx),
            SceneUpdate::Change(new_scene) => {self.scene = new_scene}
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if self.draw_fps {
            let fps = Text::new(format!("{}", fps(ctx) as u32));
            draw(ctx, &fps, DrawParam::new())?;
        }

        self.scene.draw(ctx)?;
        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
        self.scene.mouse_motion_event(ctx, x, y, xrel, yrel);
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.scene.mouse_button_down_event(ctx, button, x, y);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.scene.mouse_button_up_event(ctx, button, x, y);
    }

    fn gamepad_button_up_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
        self.scene.gamepad_button_up_event(ctx, btn, id);
    }

    fn gamepad_button_down_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
        self.scene.gamepad_button_down_event(ctx, btn, id);
    }

    fn gamepad_axis_event(&mut self, ctx: &mut Context, axis: Axis, value: f32, id: GamepadId) {
        self.scene.gamepad_axis_event(ctx, axis, value, id);
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymod: KeyMods, repeat: bool) {
        self.scene.key_down_event(ctx, keycode, keymod, repeat)
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.scene.key_up_event(ctx, keycode, keymod)
    }
}
