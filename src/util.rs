#![allow(dead_code)]
use crate::config::CONF;

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
        (year + CONF.play_speed * 0.5) /
        CONF.display_update_rate).floor()
        * CONF.display_update_rate;
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

pub fn wa_index(vec: &Vec<f64>, index: f64, width: f64) -> f64 {
    let start_index: usize = f64::max(0.0, (index - width).ceil(), ) as usize;
    let end_index: usize = f64::min((vec.len() - 1) as f64, (index + width).floor()) as usize;
    let (mut counter, mut summer): (f64, f64) = (0.0, 0.0);

    for i in start_index..end_index {
        let val: f64 = vec[i];
        let weight: f64 = 0.5 + 0.5 * ((i as f64 - index) / width * std::f64::consts::PI).cos();

        counter += weight; summer += val * weight;
    }

    summer / counter
}

pub fn wa_index_i(vec: &Vec<i32>, index: f64, width: f64) -> f64 {
    let vec_f: Vec<f64> = vec.iter().map(|&x| x as f64).collect();
    wa_index(&vec_f, index, width)
}

pub fn wa_index_i_erp(vec: &Vec<i32>, index: f64, width: f64) -> f64 {
    wa_index_i(vec, index * CONF.rank_interp as f64, width * CONF.rank_interp as f64)
}