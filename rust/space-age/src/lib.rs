// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            seconds: s
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        (1.0/31557600.0) * d.seconds as f64
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let mercury_earth_year = 31557600.0 * 0.2408467 as f64;
        (1.0/mercury_earth_year) * d.seconds as f64
    }
}

impl Planet for Venus {}
impl Planet for Earth {}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
