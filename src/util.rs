mod config;

pub fn safe_vec(vec: &Vec<f64>, i: i32) -> f64 {
    use std::cmp;
    let i2: i32 = cmp::min(cmp::max(i, 0), vec.len() as i32-1);
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
        (year + json("PLAY_SPEED") * 0.5) /
        json("DISPLAY_UPDATE_RATE") * json("DISPLAY_UPDATE_RATE")
    ).floor() as f64;
    vec_lookup(vec, rounded_year)
}