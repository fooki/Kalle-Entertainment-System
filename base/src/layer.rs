use ggez::event::MouseButton;
use ggez::{Context, GameResult};

pub trait Layer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _btn: MouseButton, _x: f32, _y: f32) {}
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _btn: MouseButton, _x: f32, _y: f32) {}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _xrel: f32, _yrel: f32) {}
}

pub struct LayerMachine {
    layers: Vec<Box<dyn Layer>>
}

impl LayerMachine {
    fn new() -> LayerMachine {
        let layers = Vec::new();
        Self { layers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLayer {}
    impl Layer for MockLayer {}

    #[test]
    fn test_new_does_not_crash() {
        LayerMachine::new();
    }
}
