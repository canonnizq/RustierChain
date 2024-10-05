#![allow(dead_code)]

use serde::Deserialize;
use std::fs::read_to_string;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub TEXT_ALPHA: f64,
    pub BARRIER_SCALE_UP: f64,
    pub PLAY_SPEED: f64,
    pub PANEL_COUNT_W: i32,
    pub PANEL_COUNT_H: i32,
    pub VERTICAL: bool,
    pub WINDOW_W: f32,
    pub WINDOW_H: f32,
    pub MARGIN: i32,
    pub CARD_MARGIN: i32,
    pub SWING_MULTI: i32,
    pub MAX_SWING: i32,
    pub PATH_CURVE: i32,
    pub PANEL_CURVE: i32,
    pub SCALE_FACTOR: f64,
    pub RANK_INTERP: i32,
    pub TRANSITION_TIME: f64,
    pub EPS: f64,
    pub DISPLAY_UPDATE_RATE: f64,
    pub BACKGROUND_COLOR: Vec<i32>,
    pub BARRIER_COLOR: Vec<i32>,
    pub PATH_COLOR: Vec<i32>,
    pub REGION_COLORS: Vec<Vec<i32>>
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let file: String = read_to_string("/config/config.json").expect("read config failed");
    let config: Config = serde_json::from_str(&file).expect("parse json failed");
    Ok(config)
}