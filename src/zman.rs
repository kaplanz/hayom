//! Daily zmanim.

use jiff::Timestamp;

/// Zmanim calculations.
pub enum Timepoint {
    /// Relative hour.
    ///
    /// Number of twelfths (halakhic hours) between sunrise and sunset.
    Hour(f32),
    /// Offset angle.
    ///
    /// Angle of the sun relative to an anchor point.
    Angle {
        /// Relative anchor.
        ///
        /// Anchor point from which the offset is computed.
        anchor: Anchor,
        /// Relative offset.
        ///
        /// Angle of the sun relative to the anchor point.
        offset: f32,
    },
}

/// Relative anchor.
pub enum Anchor {
    Sunrise,
    Sundown,
}

impl Timepoint {
    /// Compute the zman for a given day.
    pub fn compute(&self, day: solar::Day) -> Timestamp {
        match self {
            Timepoint::Hour(hour) => {
                // Compute day length
                let span = day.rise.duration_until(day.down);
                // Compute day offset
                let offs = span.mul_f32(hour / 12.);
                // Compute zman
                day.rise + offs
            }
            Timepoint::Angle { anchor, offset } => {
                unimplemented!()
            }
        }
    }
}

/// Halakhic times.
#[derive(Clone, Debug)]
pub enum Zman {
    /// _Alot Hashachar_.
    ///
    /// Daybreak (עֲלוֹת הַשַּׁחַר, _Alot Hashachar_) refers to when the first rays of
    /// light are visible in the morning.
    Alot,

    /// _Netz Hachama_.
    ///
    /// Sunrise (הָנֵץ הַחַמָּה, _Hanetz Hachama_) refers to when the ball of the sun
    /// rises above the horizon.
    Netz,

    /// _Shema_.
    ///
    /// _Shema_ (סוֹף זְמַן קְרִיאַת שְׁמַע, _Sof Zman Kriyat Shema_) means "end of the
    /// time to say the morning Shema."
    ///
    /// This is three halakhic hours into the day. These hours are
    /// variable/seasonal hours and refer to one twelfth of the time between
    /// [_daybreak_](Self::Alot) and [_nightfall_](Self::Tzet) (according to
    /// the Magen Avraham) or one twelfth of the time between
    /// [_sunrise_](Self::Netz) and [_sunset_](Self::Shekiah) (according to the
    /// Vilna Gaon).
    Shema,

    /// _Tefilla_.
    ///
    /// _Shacharit_ (סוֹף זְמַן תְּפִלָּה, _Sof Zman Tefilla_) means "end of the time to
    /// say the Shacharit Amidah."
    ///
    /// This is four halachic hours into the day. Since the Amidah is only
    /// rabbinically required (unlike the Shema which is Scriptually mandated)
    /// it is common to rely on the later time (Vilna Gaon), thus only a few
    /// calendars publish the earlier time (Magen Avraham).
    Tefilla,

    /// _Chatzot_.
    ///
    /// Midday (חֲצוֹת הַיּוֹם, _Chatzot Hayom_) means the midpoint
    /// between sunrise and sunset, or equivalently between daybreak and
    /// sundown.
    Chatzot,

    /// _Mincha Gedola_.
    ///
    /// _Mincha Gedola_ (מִנְחָה גְּדוֹלָה, literally the greater _Mincha_), one-half
    /// variable hour after midday (6.5 variable hours into the day), is the
    /// earliest time to recite _Mincha_, although one should try, if possible,
    /// to wait until [_Mincha Ketana_](Self::MinchaKetana).
    MinchaGedola,

    /// Mincha Ketana.
    ///
    /// _Mincha Ketana_ (מִנְחָה קְטַנָּה, literally the smaller _Mincha_), two and
    /// one-half variable hours before sunset, is the preferable earliest time
    /// to recite _Mincha_.
    MinchaKetana,

    /// Plag HaMincha.
    ///
    /// Plag HaMincha (פְּלַג הַמִּנְחָה, literally half of the _Mincha_) is the
    /// midpoint between [_Mincha Ketana_](Self::MinchaKetana) and
    /// [sunset](Self::Shekiah), i.e. one and one-quarter variable hours before
    /// sunset.
    PlagHaMincha,

    /// _Shekiah_.
    ///
    /// Sundown (שְׁקִיעַת הַחַמָּה, _Shkiyat Hachama_), or "sundown" is the time at
    /// which the ball of the sun falls below the horizon. The next day of the
    /// Hebrew calendar begins at this point (or shortly thereafter, at
    /// [_nightfall_](Self::Tzet) for most purposes.
    Shekiah,

    /// _Tzet Hakochavim_.
    ///
    /// Nightfall (צֵאת הַכּוֹֹכָבִים, _Tzet Hakochavim_) is the point after which it
    /// is considered definitely the following day.
    Tzet,
}

#[rustfmt::skip]
impl From<Zman> for Timepoint {
    fn from(value: Zman) -> Self {
        match value {
            Zman::Alot         => Timepoint::Angle {
                anchor: Anchor::Sunrise,
                offset: 90. - 16.1,
            },
            Zman::Netz         => Timepoint::Hour(0.),
            Zman::Shema        => Timepoint::Hour(3.),
            Zman::Tefilla      => Timepoint::Hour(4.),
            Zman::Chatzot      => Timepoint::Hour(6.),
            Zman::MinchaGedola => Timepoint::Hour(6.5),
            Zman::MinchaKetana => Timepoint::Hour(9.5),
            Zman::PlagHaMincha => Timepoint::Hour(10.75),
            Zman::Shekiah      => Timepoint::Hour(12.),
            Zman::Tzet         => Timepoint::Angle {
                anchor: Anchor::Sundown,
                offset: 8.5,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use jiff::civil::Date;
    use solar::Geo;

    use super::*;

    #[test]
    fn it_works() {
        // Declare date and place
        let date = Date::constant(2025, 11, 04);
        let place = Geo {
            lat: 43.70643,
            lon: -79.39864,
            elv: 0.,
        };

        // Calculate suntimes
        let day = solar::Day::new(date, place).unwrap();

        // Declare zmanim
        [
            (
                Zman::Alot,
                "2025-11-04T05:32-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::Netz,
                "2025-11-04T06:59-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::Shema,
                "2025-11-04T09:31-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::Tefilla,
                "2025-11-04T10:21-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::Chatzot,
                "2025-11-04T12:02-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::MinchaGedola,
                "2025-11-04T12:28-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::MinchaKetana,
                "2025-11-04T15:00-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::PlagHaMincha,
                "2025-11-04T16:03-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::Shekiah,
                "2025-11-04T18:06-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
            (
                Zman::Tzet,
                "2025-11-04T17:51-05:00[America/Toronto]"
                    .parse::<Timestamp>()
                    .unwrap(),
            ),
        ]
        // Compute zmanim
        .map(|(zman, expect)| {
            (
                zman.clone(),
                Timepoint::from(zman)
                    .compute(day.clone())
                    .round(jiff::TimestampRound::new().smallest(jiff::Unit::Minute))
                    .unwrap(),
                expect,
            )
        })
        // Ensure matches expectation
        .into_iter()
        .for_each(|(zman, calc, want)| assert_eq!(calc, want, "mismatch for `{zman:?}`"));
    }
}
