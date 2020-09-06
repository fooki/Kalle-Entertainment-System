use ggez::event::MouseButton;
use ggez::{Context, GameResult};

pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _btn: MouseButton, _x: f32, _y: f32) {}
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _btn: MouseButton, _x: f32, _y: f32) {}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _xrel: f32, _yrel: f32) {}
}
