const EARTH_RADIUS_KILOMETER: f64 = 6371.0;

fn round(val: f64) -> f64 {
    (100.0 * val).round() / 100.0
}

pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

pub fn haversine(start: &Location, end: &Location) -> f64 {
    let φ1: f64 = start.lat.to_radians();
    let φ2: f64 = end.lat.to_radians();
    let δφ: f64 = (end.lat - start.lat).to_radians();
    let δλ: f64 = (end.lon - start.lon).to_radians();

    let a: f64 = (δφ / 2.0).sin() * (δφ / 2.0).sin()
        + φ1.cos() * φ2.cos() * (δλ / 2.0).sin() * (δλ / 2.0).sin();
    let c: f64 = 2.0 * (a.sqrt()).asin();

    round(EARTH_RADIUS_KILOMETER * c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_correnct_distance_in_km() {
        let start: Location = Location {
            lat: 59.890465,
            lon: 10.523493,
        };

        let end: Location = Location {
            lat: 59.904499,
            lon: 10.786372,
        };

        assert_eq!(14.74, haversine(&start, &end));
    }
}
