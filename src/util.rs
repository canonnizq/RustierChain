fn safe_vec(vec: Vec<f64>, i: i32) -> f64 {
    use std::cmp;
    let i2: i32 = cmp::min(cmp::max(i, 0), vec.len() as i32-1);
    vec[i2 as usize]
}

fn vec_lookup(vec: Vec<f64>, year: f64) -> f64 {
    todo!("implement vec_lookup")
}