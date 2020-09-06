#[macro_use] extern crate log;

use std::path;
use ggez::mint::Point2;
use clap::Clap;
use ggez::{graphics, Context, GameResult};
use ggez::conf::{self, FullscreenType};
use ggez::ContextBuilder;
use ggez::event::{self};
use ggez::graphics::{draw, Text, DrawParam, Font, TextFragment, Scale};
use ggez::event::MouseButton;

struct MainMenu {
    text: Text
}

impl MainMenu {
    pub fn new(ctx: &mut Context) -> MainMenu {
        let fancy_font = Font::new(ctx, "/boxy.ttf").expect("boom");
        let text = Text::new(
            TextFragment::new("Hey Emelie!")
                .font(fancy_font)
                .scale(Scale::uniform(80.0))
        );
        MainMenu { text }
    }
}


impl base::Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        draw(ctx, &self.text, DrawParam::new().dest(Point2{x: 300.0, y: 300.0}))?;
        Ok(())
    }
}

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
        .build()
        .expect("aieee, could not create ggez context!");

    let main_menu = MainMenu::new(&mut ctx);
    let mut game = base::Game::new(&mut ctx, opts.fps, Box::new(main_menu));

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
