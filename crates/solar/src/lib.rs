use jiff::civil::Date;
use jiff::{Error, Timestamp};

/// Suntimes calculations.
mod calc;

/// Suntimes times for a given day.
#[derive(Debug)]
pub struct Day {
    /// Calendar date.
    pub date: Date,
    /// Sunrise time.
    pub rise: Timestamp,
    /// Sunset time.
    pub down: Timestamp,
    /// Private field.
    _prv: (),
}

impl Day {
    /// Constructs a new `Day`.
    pub fn new(date: Date, place: Geo) -> Result<Self, Error> {
        calc::suntimes(date, place)
    }
}

/// A geographic coordinate on Earth.
pub struct Geo {
    /// Longitude (coordinate).
    pub lon: f64,
    /// Latitude (coordinate).
    pub lat: f64,
    /// Elevation (meters).
    pub elv: f64,
}
