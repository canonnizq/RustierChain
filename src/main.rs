use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

use std::fs;
use serde_json;

fn main() {

    let config: String = fs::read_to_string("config/config.json").expect("nonexistent file");
    let json: serde_json::Value = serde_json::from_str(&config).expect("bad json");

    let (mut ctx, event_loop) = ContextBuilder::new("RustierChain", "CanonNi").build().expect("ggez creation failed");
    let game = MainState::new(&mut ctx);
    event::run(ctx, event_loop, game);

}

struct MainState {

}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {

    }
}

impl EventHandler for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

}