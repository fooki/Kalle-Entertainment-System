use ggez::{Context};
use ggez::graphics::{window};

pub fn window_height(ctx: &mut Context) -> f32 {
    window(ctx).get_inner_size().unwrap().height as f32
}


pub fn window_width(ctx: &mut Context) -> f32 {
    window(ctx).get_inner_size().unwrap().width as f32
}

pub fn center(ctx: &mut Context) -> (f32, f32) {
    (window_width(ctx)/2.0, window_height(ctx)/2.0)
}

use std::sync::atomic::{AtomicUsize, Ordering};
pub fn next_id() -> i64 {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed) as i64
}
