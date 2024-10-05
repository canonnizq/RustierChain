#![allow(dead_code)]

use serde::Deserialize;
use std::fs::read_to_string;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub text_alpha: f64,
    pub barrier_scale_up: f64,
    pub play_speed: f64,
    pub panel_count_w: i32,
    pub panel_count_h: i32,
    pub vertical: bool,
    pub window_w: f32,
    pub window_h: f32,
    pub margin: i32,
    pub card_margin: i32,
    pub swing_multi: f64,
    pub max_swing: i32,
    pub path_curve: i32,
    pub panel_curve: i32,
    pub scale_factor: f64,


    pub rank_interp: i32,
    pub transition_time: f64,
    pub eps: f64,
    pub display_update_rate: f64,

    pub background_color: Vec<i32>,
    pub barrier_color: Vec<i32>,
    pub path_color: Vec<i32>,
    pub region_colors: Vec<Vec<i32>>
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let file: String = read_to_string("/config/config.json").expect("read config failed");
    let file: String = read_to_string("/config/config.json").expect("read config failed");
    let config: Config = serde_json::from_str(&file).expect("parse json failed");
    Ok(config)
}