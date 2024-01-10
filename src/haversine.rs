use std::f64::{consts::PI};
const R: f64 = 6371000.0; //радиус Земли в метрах

pub fn haversine_formula(point_start: Point, point_end: Point) -> f64
{
    let lon1 = point_start.lon;
    let lat1 = point_start.lat;
    let lon2 = point_end.lon;
    let lat2 = point_end.lat;

    let lat = deg_to_rad(lat2 - lat1);
    let lon = deg_to_rad(lon2 - lon1);

    let root_val = f64::sin(lat/2.0) * f64::sin(lat/2.0) + f64::cos(deg_to_rad(lat2) * f64::sin(lon/2.0) * f64::sin(lon/2.0));

    let ret_val: f64 = 2.0 * R * f64::atan(f64::sqrt(root_val) - f64::sqrt(1.0 - root_val));

    return ret_val;
}

pub(crate) struct Point {
    pub(crate) lon: f64,
    pub(crate) lat: f64
}

fn deg_to_rad(deg: f64) -> f64 {
    return deg * (PI / 180.0)
}