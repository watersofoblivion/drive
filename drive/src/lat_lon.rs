// #[cfg_attr(test, derive(Debug, PartialEq, Clone))]
#[derive(Debug, PartialEq, Clone)]
pub struct LatLon {
    lat: f32,
    lon: f32,
}

impl LatLon {
    pub fn new(lat: f32, lon: f32) -> Self {
        Self { lat, lon }
    }
}

impl LatLon {
    pub fn shift(&self, lat: f32, lon: f32) -> Self {
        Self {
            lat: self.lat + lat,
            lon: self.lon + lon,
        }
    }
}

#[cfg(test)]
impl LatLon {
    pub fn assert_lat(&self, expected: impl Into<f32>) {
        let expected = expected.into();
        let actual = self.lat;

        assert_eq!(
            expected, actual,
            "Expected latitude to be {:?}, found {:?}",
            expected, actual
        )
    }

    pub fn assert_lon(&self, expected: impl Into<f32>) {
        let expected = expected.into();
        let actual = self.lon;

        assert_eq!(
            expected, actual,
            "Expected longitude to be {:?}, found {:?}",
            expected, actual
        )
    }
}
