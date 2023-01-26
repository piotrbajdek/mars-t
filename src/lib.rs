// The Universal Permissive License (UPL), Version 1.0
// Copyright © 2023 Piotr Bajdek

// LIBRARY

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

use hifitime::{Epoch, TimeUnits};
use std::str::FromStr;

// FUNCTIONS

// CURRENT DATA AND TIME

pub fn msd_now_flt() -> f64 {
    let dt_tm = Epoch::now().unwrap();
    let jde_tt = dt_tm.to_jde_tt_days();

    let mars_sol_date_now_float: f64 = (jde_tt - 2405522.0028779) / 1.0274912517;
    return mars_sol_date_now_float;
}

pub fn mtc_now_flt() -> f64 {
    let mars_sol_date_now_float = msd_now_flt();

    let martian_coordinated_time_now_float: f64 = mars_sol_date_now_float.rem_euclid(1.0) * 24.0;
    return martian_coordinated_time_now_float;
}

pub fn mtc_now_fmt() -> String {
    let martian_coordinated_time_now_float = mtc_now_flt();

    let martian_coordinated_time_now_formatted = TimeUnits::hours(martian_coordinated_time_now_float);
    return martian_coordinated_time_now_formatted.to_string();
}

// CUSTOMISED DATE AND TIME

pub fn msd_flt(date: &str, time: &str, scale: &str) -> f64 {
    let date_time_scale = date.to_owned() + " " + time + " " + scale;

    let dt_tm = Epoch::from_str(&date_time_scale);
    let jde_tt = dt_tm.unwrap().to_jde_tt_days();

    let mars_sol_date_float: f64 = (jde_tt - 2405522.0028779) / 1.0274912517;
    return mars_sol_date_float;
}

pub fn mtc_flt(date: &str, time: &str, scale: &str) -> f64 {
    let mars_sol_date_float = msd_flt(date, time, scale);

    let martian_coordinated_time_float: f64 = mars_sol_date_float.rem_euclid(1.0) * 24.0;
    return martian_coordinated_time_float;
}

pub fn mtc_fmt(date: &str, time: &str, scale: &str) -> String {
    let martian_coordinated_time_float = mtc_flt(date, time, scale);

    let martian_coordinated_time_formatted = TimeUnits::hours(martian_coordinated_time_float);
    return martian_coordinated_time_formatted.to_string();
}
