mod conv;

/// Hebrew months.
pub enum Month {
    Nisan = 1,
    Iyyar,
    Sivan,
    Tamuz,
    Av,
    Elul,
    Tishrei,
    Cheshvan,
    Kislev,
    Tevet,
    Shvat,
    Adar1,
    Adar2,
}

impl Month {
    /// Length, in days, of a lunar month.
    #[expect(unused)]
    const LENGTH: f64 = 765433. / 25920.;
}
