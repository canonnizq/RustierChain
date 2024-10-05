#![allow(dead_code)]

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowSetup, WindowMode};

use std::collections::HashMap;

mod util; 
mod config;
mod item;
use crate::config::CONF;
use crate::item::Item;


struct MainState {
    current_year: f64,
    items: Vec<Item>,
    year_index: HashMap<String, usize>,
    //data: Vec<String>,
}

impl MainState {

    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState{
            current_year: 1790.0,
            items: Vec::new(),
            year_index: HashMap::new(),
            //data: vec![read_to_string("data/states.tsv").expect("read tsv failed")],
        })
    }

    pub fn calculate_barriers(&self) -> Vec<f64> {
        let mut barriers: Vec<f64> = vec![0.0; self.items.len()];
        let mut ordered_items: Vec<Item> = Vec::new();
        
        for item in &self.items {
            let value: f64 = item.value_cache;
            let index: usize = ordered_items
                .binary_search_by(|p: &Item| p.value_cache.partial_cmp(&value).unwrap())
                .unwrap_or_else(|x: usize| x);
            ordered_items.insert(index, item.clone());
        }
        
        barriers[0] = 0.0;
        for i in 1..self.items.len() {
            let vp = ordered_items[i-1].value_cache;
            let vt = ordered_items[i].value_cache;
            barriers[i] = (vp - vt) / vp;
        }
        barriers
    }

}

impl EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.current_year += 1.0 / 90.0;
        if self.current_year >= self.items.len() as f64 {
            ctx.request_quit()
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas: Canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        let _barriers: Vec<f64> = self.calculate_barriers();
        // self.draw_path(ctx, &barriers)?;

        canvas.finish(ctx)
    }
    /*
    fn draw_path(&self, _ctx: &mut Context, barriers: &Vec<f64>) -> GameResult {
        for i in 0..self.items.len() {
            todo!("implement draw logic for path and barriers")
        }
        Ok(())
    }
    */
}

fn main() {

    let (mut ctx, event_loop) = ContextBuilder::new("RustierChain", "CanonNi")
        .window_setup(WindowSetup::default().title("RustierChain"))
        .window_mode(WindowMode::default().dimensions(CONF.window_w, CONF.window_h))
        .build().expect("ggez creation failed");

    let state: MainState = MainState::new(&mut ctx).expect("state creation failed");
    event::run(ctx, event_loop, state);

}