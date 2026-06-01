#[derive(Debug, Clone)]
pub struct GeoTool;

impl GeoTool {
    pub fn new() -> Self { Self }

    /// Calculates Haversine distance between two points in km
    pub fn distance_km(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6371.0; // Earth radius in km
        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();
        let a = (d_lat / 2.0).sin().powi(2) +
                lat1.to_radians().cos() * lat2.to_radians().cos() *
                (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        r * c
    }
}

impl Default for GeoTool {
    fn default() -> Self { Self::new() }
}
