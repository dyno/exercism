#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const EARTH_YEARS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0 / Self::EARTH_YEARS
    }
}

pub struct Mercury;
impl Planet for Mercury {
    const EARTH_YEARS: f64 = 0.2408467;
}
pub struct Venus;
impl Planet for Venus {
    const EARTH_YEARS: f64 = 0.61519726;
}
pub struct Earth;
impl Planet for Earth {
    const EARTH_YEARS: f64 = 1.0;
}
pub struct Mars;
impl Planet for Mars {
    const EARTH_YEARS: f64 = 1.8808158;
}
pub struct Jupiter;
impl Planet for Jupiter {
    const EARTH_YEARS: f64 = 11.862615;
}
pub struct Saturn;
impl Planet for Saturn {
    const EARTH_YEARS: f64 = 29.447498;
}
pub struct Uranus;
impl Planet for Uranus {
    const EARTH_YEARS: f64 = 84.016846;
}
pub struct Neptune;
impl Planet for Neptune {
    const EARTH_YEARS: f64 = 164.79132;
}
