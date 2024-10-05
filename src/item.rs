use crate::config::CONF;
use crate::util;

#[derive(Clone)]
pub struct Item {
    name: String,
    start_year: String,
    values: Vec<f64>,
    first_nonzero: i32,
    ranks: Vec<i32>,
    id: i32,
    pub value_cache: f64,
    velocity_cache: f64,
    col: i32,
    map_coor: Vec<i32>,
    totals: Vec<f64>,
    current_year: f64,
}

impl Item {
    pub fn new(&mut self, dataline: String, len: i32, t_id: i32) {
        let parts: Vec<&str> = dataline.split('\t').collect();
        let rlen: i32 = len * CONF.rank_interp;

        (self.first_nonzero, self.map_coor) = (-1, vec![0, 2]);
        self.name = parts[0].to_string();
        self.col = parts[1].parse().expect("parse str to i32 failed");
        self.values = vec![0.0; len as usize];
        self.ranks = vec![0; rlen as usize];
        self.id = t_id;

        for y in 0..len {
            let y: usize = y as usize;
            self.values[y] = parts[y+2].parse().expect("parse str to f64 failed");
            self.totals[y] += self.values[y];

            if self.first_nonzero == -1 && self.values[y] > 0.0 {
                self.first_nonzero = y as i32
            }
        }

        for y_r in 0..rlen {
            self.ranks[y_r as usize] = 0
        }
    }

    pub fn calculate_value(&mut self) -> f64 {
        self.value_cache = util::vec_lookup(&self.values, self.current_year);
        self.value_cache
    }

    pub fn calculate_velocity(&mut self, y: f64) -> f64 {
        let a_rank_pre: f64 = util::wa_index_i_erp(&self.ranks, y - CONF.eps, CONF.transition_time);
        let a_rank_post: f64 = util::wa_index_i_erp(&self.ranks, y + CONF.eps, CONF.transition_time);
        self.velocity_cache = (a_rank_post - a_rank_pre) / (CONF.eps * 2.0);
        self.velocity_cache
    }

}