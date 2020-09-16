use ggez::event::{MouseButton, Button, GamepadId, Axis};
use ggez::input::keyboard::{KeyMods, KeyCode};
use ggez::{Context, GameResult};

pub enum SceneUpdate {
    Nothing,
    Quit,
    Change(Box<dyn Scene>)
}

pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<SceneUpdate>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _btn: MouseButton, _x: f32, _y: f32) {}
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _btn: MouseButton, _x: f32, _y: f32) {}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _xrel: f32, _yrel: f32) {}

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {}
    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {}
    fn gamepad_axis_event(&mut self, _ctx: &mut Context, _axis: Axis, _value: f32, _id: GamepadId) {}

    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {}
    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymod: KeyMods) {}
}
