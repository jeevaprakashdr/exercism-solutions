// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_SECONDS : Duration = Duration(31557600);

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}


pub trait Planet {
    const PLANET_YEAR : f64;    
    
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / Self::PLANET_YEAR
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
        $orbital_period: literal
    ) => {
        impl Planet for $planet_name {
            const PLANET_YEAR: f64 = EARTH_SECONDS.0 as f64 * $orbital_period as f64;
        }
    };
}
planet_impl!(Earth, 1.0);
planet_impl!(Mercury, 0.2408467);
planet_impl!(Venus, 0.61519726);
planet_impl!(Mars, 1.8808158);
planet_impl!(Jupiter, 11.862615);
planet_impl!(Saturn, 29.447498);
planet_impl!(Uranus, 84.016846);
planet_impl!(Neptune, 164.79132);
