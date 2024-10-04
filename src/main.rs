use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowSetup, WindowMode};

use std::collections::HashMap;
use std::fs::read_to_string;
use serde_json::{Value, from_str};

mod config;
mod util;

fn json(key: &str) -> Option<Value> {
    let data = read_to_string("config/config.json").expect("bad path");
    let v: Value = from_str(&data).expect("bad json");
    v.get(key).cloned()
}

#[derive(Clone)]
struct Item {
    values: Vec<f64>,
    first_nonzero: i32,
    ranks: Vec<usize>,
    velocity_cache: f64,
    value_cache: f64,
}

impl Item {
    
    pub fn calculate_value(&mut self, current_year: f64) -> f64 {
        self.value_cache = self.values[current_year as usize];
        self.value_cache
    }
    pub fn calculate_velocity(&mut self, _current_year: f64) -> f64 {
        self.velocity_cache = 0.0;
        self.velocity_cache
    }
    fn draw_panel(&self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    
}

struct MainState {
    current_year: f64,
    items: Vec<Item>,
    year_index: HashMap<String, usize>,
    data: Vec<String>,
}

impl MainState {

    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState{
            current_year: 1790.0,
            items: Vec::new(),
            year_index: HashMap::new(),
            data: vec![read_to_string("data/states.tsv").expect("read tsv failed")],
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
        let mut canvas: Canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        let barriers: Vec<f64> = self.calculate_barriers();
        // self.draw_path(ctx, &barriers)?;
        
        for item in &self.items {
            item.draw_panel(ctx)?;
        }

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
        .window_mode(WindowMode::default().dimensions(1920.0, 1080.0))
        .build().expect("ggez creation failed");

    let state: MainState = MainState::new(&mut ctx).expect("state creation failed");
    event::run(ctx, event_loop, state);

}