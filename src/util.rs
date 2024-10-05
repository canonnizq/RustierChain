#![allow(dead_code)]

use crate::config;
use once_cell::sync::Lazy;
static CONF: Lazy<config::Config> = Lazy::new(|| {
    config::load_config().expect("load config failed")
});

pub fn safe_vec(vec: &Vec<f64>, i: i32) -> f64 {
    let i2: i32 = i32::min(i32::max(i, 0), vec.len() as i32-1);
    vec[i2 as usize]
}

pub fn vec_lookup(vec: &Vec<f64>, year: f64) -> f64 {
    let year_int: i32 = year.floor() as i32;
    let before: f64 = safe_vec(vec, year_int);
    let after: f64 = safe_vec(vec, year_int + 1);
    before + (after - before) * year % 1.0
}

pub fn display_slowed_vec_lookup(vec: &Vec<f64>, year: f64) -> f64 {
    let rounded_year: f64 = (
        (year + CONF.PLAY_SPEED * 0.5) /
        CONF.DISPLAY_UPDATE_RATE).floor()
        * CONF.DISPLAY_UPDATE_RATE;
    vec_lookup(vec, rounded_year)
}

pub fn commafy(f: f64) -> String {
    let s: String = f.round().to_string();
    let mut res: String = String::new();
    for (i, c) in s.chars().enumerate() {
        if (s.len() - i) % 3 == 0 && i != 0 {
            res.push(',')
        }
        res.push(c)
    }
    res
}

pub fn vec_to_text(vec: &Vec<f64>, year: f64) -> String {
    commafy(vec_lookup(vec, year).round())
}

pub fn cap(val: f64, limit: f64) -> f64 {
    f64::min(f64::max(val, -limit), limit)
}