// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECONDS:f64 = 31557600.0;
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
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
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

macro_rules! planet_impl {
    (
        $planet_name: ident,
        $orbital_period: expr
    ) => {
        impl Planet for $planet_name {
            fn years_during(d: &Duration) -> f64 {
                let earth_year: f64 = EARTH_YEAR_IN_SECONDS * $orbital_period as f64;
                (1.0/earth_year) * d.seconds as f64
            }
        }        
    };
}
planet_impl!(Earth, 1.0);
planet_impl!(Mercury, 0.2408467);
planet_impl!(Venus, 0.61519726);

impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
