use jiff::civil::Date;
use jiff::tz::TimeZone;
use jiff::{Error, SignedDuration, Timestamp};

use crate::{Day, Geo};

/// Convert timestamp to Julian date.
fn ts2j(ts: Timestamp) -> f64 {
    ts.as_duration().as_secs_f64() / 86400. + 2440587.5
}

/// Convert Julian date to timestamp.
fn j2ts(j: f64) -> Result<Timestamp, Error> {
    Timestamp::from_duration(SignedDuration::from_secs_f64((j - 2440587.5) * 86400.))
}

/// Calculate sunrise and sunset times.
///
/// See more [here].
///
/// [here]: https://en.wikipedia.org/wiki/Sunrise_equation
pub fn suntimes(date: Date, place: Geo) -> Result<Day, Error> {
    // Fix timestamp to noon
    let tnoon = date.at(12, 0, 0, 0).to_zoned(TimeZone::UTC)?.timestamp();

    // Julian day
    let jdate = ts2j(tnoon);
    let day_n = (jdate - (2451545. + 0.0009) + 69.184 / 86400.).ceil();

    // Mean solar time
    let jstar = day_n + 0.0009 - place.lon / 360.;

    // Solar mean anomaly
    let m_deg = (357.5291 + 0.98560028 * jstar) % 360.;
    let m_rad = m_deg.to_radians();

    // Equation of the center
    let c_deg =
        1.9148 * (1. * m_rad).sin() + 0.0200 * (2. * m_rad).sin() + 0.0003 * (3. * m_rad).sin();

    // Ecliptic longitude
    let l_deg = (m_deg + c_deg + 180. + 102.9372) % 360.;
    let l_rad = l_deg.to_radians();

    // Solar transit
    let trans = 2451545.0 + jstar + 0.0053 * m_rad.sin() - 0.0069 * (2. * l_rad).sin();

    // Declination of the sun
    let sin_d = l_rad.sin() * 23.4397_f64.to_radians().sin();
    let cos_d = sin_d.asin().cos();

    // Hour angle
    let cos_w = {
        let lat = place.lat.to_radians();
        let elv = (-0.833_f64 - (2.076 * place.elv.sqrt() / 60.)).to_radians();
        (elv.sin() - lat.sin() * sin_d) / (lat.cos() * cos_d)
    };
    let w_rad = cos_w.acos();
    let w_deg = w_rad.to_degrees();

    // Sunrise and sunset
    let jrise = trans - w_deg / 360.;
    let jdown = trans + w_deg / 360.;

    // Convert to timestamp
    let rise = j2ts(jrise)?;
    let down = j2ts(jdown)?;

    Ok(Day {
        date,
        rise,
        down,
        _prv: (),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Declare date and place
        let date = Date::constant(2025, 10, 10);
        let place = Geo {
            lat: 33.00801,
            lon: 35.08794,
            elv: 0.,
        };

        // Calculate suntimes
        let day = suntimes(date, place).unwrap();

        // Ensure matches expectation
        assert_eq!(day.rise, Timestamp::constant(1760067670, 032991409));
        assert_eq!(day.down, Timestamp::constant(1760109268, 838135958));
    }
}
