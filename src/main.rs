use ggez::{Context, ContextBuilder, GameResult};
//use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowSetup, WindowMode};

use std::collections::HashMap;
use std::fs::read_to_string;
//use serde_json::from_str;

struct Item {
    values: Vec<f64>,
    first_nonzero: i32,
    ranks: Vec<usize>,
    velocity_cache: f64,
    value_cache: f64,
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
            data: vec![read_to_string("/data/states.tsv").expect("read tsv failed")],
        })
    }

    pub fn calculate_barriers(&self) -> Vec<f64> {
        let mut barriers = vec![0.0; self.items.len()];
        let mut ordered_items: Vec<Item> = Vec::new();
        
        for item in &self.items {
            let value: f64 = item.value_cache;
            let index: usize = ordered_items
                .binary_search_by(|p: &Item| p.value_cache.partial_cmp(&value).unwrap())
                .unwrap_or_else(|x: usize| x);
            ordered_items.insert(index, item);
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

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        //let barriers: f64 = calculate_barriers();
        Ok(())
    }

}

fn main() {

    let (mut ctx, event_loop) = ContextBuilder::new("RustierChain", "CanonNi")
        .window_setup(WindowSetup::default().title("RustierChain"))
        .window_mode(WindowMode::default().dimensions(1920.0, 1080.0))
        .build().expect("ggez creation failed");

    let state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, state);

}