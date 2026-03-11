//! # Measurements
//!
//! A `Measurement` is a snapshot of telemetry data at a single point in time.
//! It captures all of the individual data points that were collected from the
//! vehicle together into one structure.

/*
 * Imports
 */

// Stdlib

// External Libraries
use chrono::{DateTime, Duration, TimeZone};
use uom::si::f32::*;

// Internal
use super::lat_lon::LatLon;

/*
 * Type
 */

///
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct Measurement<'a, Z>
where
    Z: TimeZone,
{
    /// The timestamp the measurement was recorded.
    timestamp: DateTime<Z>,

    /// The latitude and longitude where the measurement was recorded.
    position: LatLon,

    /// The speed the vehicle was going.
    speed: Velocity,

    /// The heading of the vehicle
    heading: f32,

    /// User provided notes for the data point
    notes: Option<&'a str>,
}

/*
 * Implementation
 */

/// # Constructors
///
///
impl<'a, Z> Measurement<'a, Z>
where
    Z: TimeZone,
{
    /// Construct a `Measurement` from its constituent data points.
    pub fn new(
        timestamp: DateTime<Z>,
        position: LatLon,
        speed: Velocity,
        heading: f32,
        notes: Option<&'a str>,
    ) -> Self {
        Self {
            timestamp,
            position,
            speed,
            heading,
            notes,
        }
    }
}

/// # Actions
///
///
impl<'a, Z> Measurement<'a, Z>
where
    Z: TimeZone,
{
    pub fn estimate_position_after(&self, t: Duration) -> LatLon {
        let sec = t.as_seconds_f32();
        let lat_shift = self.heading.sin() * sec;
        let lon_shift = self.heading.cos() * sec;

        self.position.shift(lat_shift, lon_shift)
    }
}

/*
 * Testing
 */

#[cfg(test)]
impl<'a, Z> Measurement<'a, Z>
where
    Z: TimeZone,
{
    /// Assert the timestamp the measurement was recorded is the expected value.
    pub fn assert_timestamp<'e>(&self, expected: impl Into<&'e DateTime<Z>>)
    where
        Z: 'e,
    {
        let expected = expected.into();
        let actual = &self.timestamp;

        assert_eq!(
            expected, actual,
            "Expected the measurement's timestamp to be {:?}, found {:?}",
            expected, actual
        )
    }

    /// Assert the latitude and longitude where the measurement was recorded is
    /// the expected value.
    pub fn assert_position<'e>(&self, expected: impl Into<&'e LatLon>) {
        let expected = expected.into();
        let actual = &self.position;

        assert_eq!(
            expected, actual,
            "Expected the measurement's position to be {:?}, found {:?}",
            expected, actual
        )
    }

    /// Assert the speed the vehicle was going is the expected value.
    pub fn assert_speed(&self, expected: impl Into<Velocity>) {
        let expected = expected.into();
        let actual = self.speed;

        assert_eq!(
            expected, actual,
            "Expected the measurement's speed to be {:?}, found {:?}",
            expected, actual
        )
    }

    /// Assert the heading of the vehicle is the expected value.
    pub fn assert_heading(&self, expected: impl Into<f32>) {
        let expected = expected.into();
        let actual = self.heading;

        assert_eq!(
            expected, actual,
            "Expected the measurement's heading to be {:?}, found {:?}",
            expected, actual
        )
    }

    /// Assert that there are no user-supplied notes attached.
    pub fn assert_no_notes(&self) {
        match &self.notes {
            Some(actual) => panic!(
                "Expected measurement's notes to be empty, found {:?}",
                actual
            ),
            None => (),
        }
    }

    /// Assert that there are user-supplied notes attached and that they are the
    /// expected value.
    pub fn assert_notes<'e>(&self, expected: impl Into<&'e str>) {
        let expected = expected.into();

        match self.notes {
            Some(actual) => assert_eq!(
                expected, actual,
                "Expected measurement's notes to be {:?}, found {:?}",
                expected, actual
            ),
            None => panic!(
                "Expected measurement's notes to be {:?}, found none",
                expected
            ),
        }
    }
}

/// Unit Tests
#[cfg(test)]
mod test {
    /// External Libraries
    use chrono::Utc;
    use uom::si::velocity::mile_per_hour;

    /// Module under test
    use super::*;

    /// Test that the all-arguments constructor properly constructs a
    /// `Measurement`.
    #[test]
    fn test_new() {
        let timestamp = Utc::now();
        let position = LatLon::new(40.0190, 105.2747);
        let speed = Velocity::new::<mile_per_hour>(42.0);
        let heading = 24.0;
        let notes = "Some notes";

        let measurement = Measurement::new(
            timestamp.clone(),
            position.clone(),
            speed,
            heading,
            Some(notes),
        );
        measurement.assert_timestamp(&timestamp);
        measurement.assert_position(&position);
        measurement.assert_speed(speed);
        measurement.assert_heading(heading);
        measurement.assert_notes(notes);
    }

    #[test]
    fn test_estimate_position_after() {
        let timestamp = Utc::now();
        let position = LatLon::new(40.0190, 105.2747);
        let speed = Velocity::new::<mile_per_hour>(42.0);
        let heading = 24.0;
        let measurement =
            Measurement::new(timestamp.clone(), position.clone(), speed, heading, None);
    }
}
