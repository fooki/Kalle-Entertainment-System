#[macro_use] extern crate log;

mod button_group;
mod main_menu;
mod tetris_game;
mod tetris_block;
mod figure_state;
mod figure;
mod board;

use std::path;

use clap::Clap;

use ggez::conf::{self, FullscreenType};
use ggez::ContextBuilder;
use ggez::event::{self};
use ggez::conf::ModuleConf;
use main_menu::MainMenu;

#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    fps: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    env_logger::init();

    let resource_dir = path::PathBuf::from("./resources");
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Karl Johansson")
        .add_resource_path(resource_dir)
        .window_mode(conf::WindowMode::default().fullscreen_type(FullscreenType::Windowed).dimensions(1024.0, 768.0))
        .modules(ModuleConf { gamepad: true, audio: true})
        .build()
        .expect("aieee, could not create ggez context!");

    let main_menu = MainMenu::new(&mut ctx);
    let mut game = base::Game::new(&mut ctx, opts.fps, Box::new(main_menu));

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
