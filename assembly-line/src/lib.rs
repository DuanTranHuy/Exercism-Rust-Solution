// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed_f64 = f64::from(speed);
    let car_per_hour_f64 = f64::from(221);

    match speed {
        0..=4 => return speed_f64 * car_per_hour_f64,
        5..=8 => return speed_f64 * car_per_hour_f64 * 0.9 as f64,
        _ => return speed_f64 * car_per_hour_f64 * 0.77 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let product =  production_rate_per_hour(speed);
    return (product / 60.0) as u32
}
