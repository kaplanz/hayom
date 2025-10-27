use std::ops::RangeInclusive;

use jiff::ToSpan;
use jiff::civil::{Date as Greg, Era};
use thiserror::Error;

/// Period during which the calendar was switched from Julian to Gregorian. All
/// dates within this period (exclusive) are invalid.
const ADJ: RangeInclusive<Greg> = Greg::constant(1752, 09, 02)..=Greg::constant(1752, 09, 14);

/// Rata Die date number.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RataDie(u64);

impl TryFrom<Greg> for RataDie {
    type Error = Error;

    fn try_from(greg: Greg) -> Result<Self, Self::Error> {
        // Ensure era is CE
        let (py, Era::CE) = greg.era_year() else {
            return Err(Error::Era);
        };

        // Ensure outside adjustment
        if ADJ.contains(&greg) {
            return Err(Error::Adj);
        }

        // Adjust Gregorian year for computation
        let py = u64::try_from(py).unwrap() - 1;

        // Compute day number
        let mut rata: u64 = {
            // Days from previous year
            let prev = 365 * py;
            // Days from leap years
            let leap = py / 4;
            // Days this year
            let this = u64::try_from(greg.day_of_year()).unwrap();

            prev + leap + this
        };

        // Gregorian adjustment
        if &greg > ADJ.start() {
            // Remove centiry leap years
            rata -= py / 100;
            // Insert Gregorian leap years
            rata += py / 400;
        } else {
            // Absolute dates obtained from Julian dates need to be adjusted
            // because the Julian date 1/2/1 is the equivlent of Gregorian
            // 12/31/1 BC, so the number of days since Julian 12/31/1 BC is 2
            // greater than since Gregorian 12/31/1 BC.
            rata -= 2
        }

        Ok(Self(rata))
    }
}

impl From<RataDie> for Greg {
    fn from(rata: RataDie) -> Self {
        // Finds the year in which a given R.D. occurs.
        //
        // Returns the year and remaining days.
        //
        // See the footnote on page 384 of “Calendrical Calculations, Part II:
        // Three Historical Calendars” by E. M. Reingold, N. Dershowitz, and S.
        // M. Clamen, Software--Practice and Experience, Volume 23, Number 4
        // (April, 1993), pages 383-404 for an explanation.
        let (year, day) = {
            // Get the absolute date for 02 Sep 1752
            const ADJ: RataDie = RataDie(639795);

            // Subtract 1 because we are counting from Gregorian date 12/31/1 BCE
            // (Julian 1/2/1).
            let mut l0 = rata.0 - 1;

            // Number of 100 and 400 year periods and days into current period
            let n400;
            let n100;
            let day1;
            let day2;
            if rata > ADJ {
                n400 = l0 / 146097;
                day1 = l0 % 146097;
                n100 = day1 / 36524;
                day2 = day1 % 36524;
            } else {
                // Gregorian 12/31/1 BCE = Julian 1/2/1 CE, so if we are in the Julian
                // calendar, add 2 so that we are counting from Julian 12/31/1 BCE.
                l0 += 2;

                n400 = l0 / 146100;
                day1 = l0 % 146100;
                n100 = day1 / 36525;
                day2 = day1 % 36525;
            }

            // Number of 4 year periods
            let n4 = day2 / 1461;
            let d3 = day2 % 1461;
            // Number of years into current period
            let n1 = d3 / 365;
            let mut dd = d3 % 365;
            // Total years
            let mut yy = 400 * n400 + 100 * n100 + 4 * n4 + n1;

            // If we didn't get a 4-year block (with a leap day in it), but we did
            // get 4 separate years it must be December 31 on the previous year
            // (because there was a leap day). So don't add 1.
            if n100 == 4 || n1 == 4 {
                // But first, if yy is 0, then we are actually in 1 BCE
                if yy != 0 {
                    dd = 365;
                } else {
                    // Year 1 BCE was a leap year
                    panic!();
                }
            }

            // Otherwise it is the next year (because there is no year 0, so
            // generally add 1).
            yy += 1;

            (yy as i16, dd as i16)
        };

        // Calculate date from year + day of year
        Greg::new(year, 01, 01).unwrap() + day.days()
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by converting between dates.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    /// Invalid era for date.
    #[error("invalid era (must be CE)")]
    Era,
    /// Within Gregorian adjustment.
    #[error("within Gregorian adjustment")]
    Adj,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Declare date
        let greg = Greg::constant(2025, 10, 26);

        // Calculate Rata Die
        let rata = RataDie::try_from(greg).unwrap();

        // Ensure matches expectation
        assert_eq!(rata, RataDie(739550));
        assert_eq!(greg, Greg::from(rata));
    }
}
