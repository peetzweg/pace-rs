use std::{fmt::Display, ops::Add};

pub struct Meters(pub u32);

pub struct Kilometers(pub u32);

#[derive(Debug, PartialEq)]
pub struct Distance(pub u32, pub u32);

impl Add<Meters> for Meters {
    type Output = Meters;

    fn add(self, rhs: Meters) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

impl Add<Meters> for Kilometers {
    type Output = Meters;

    fn add(self, rhs: Meters) -> Self::Output {
        Meters::from(self) + rhs
    }
}

impl Add<Kilometers> for Meters {
    type Output = Meters;

    fn add(self, rhs: Kilometers) -> Self::Output {
        Meters::from(rhs) + self
    }
}

impl From<Distance> for Meters {
    fn from(distance: Distance) -> Self {
        Kilometers(distance.0) + Meters(distance.1)
    }
}

impl Distance {
    pub fn new(distance: &str) -> Option<Self> {
        match distance {
            _ if distance.ends_with("km") => {
                let v = distance
                    .trim_end_matches("km")
                    .split(".")
                    .take(2)
                    .collect::<Vec<&str>>();

                let km = v.get(0).unwrap_or(&"0");
                let m = v.get(1).unwrap_or(&"0");

                // shorten m to 3 digits
                let m = m.chars().take(3).collect::<String>();

                let km = km.parse::<u32>().ok()?;
                if let Some(m) = m.parse::<u32>().ok() {
                    return Some(Self(km, m));
                } else {
                    return Some(Self(km, 0));
                }
            }
            _ if distance.ends_with("m") => {
                let m = distance.trim_end_matches("m").parse::<u32>().ok()?;
                let km = m / 1000;
                let m = m % 1000;

                Some(Self(km, m))
            }
            _ => None,
        }
    }
}

impl From<Kilometers> for Meters {
    fn from(kms: Kilometers) -> Self {
        Meters(kms.0 * 1000)
    }
}

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_constructor() {
        assert_eq!(Distance::new("10km").unwrap(), Distance(10, 0));
        assert_eq!(Distance::new("10.20km").unwrap(), Distance(10, 20));
        assert_eq!(Distance::new("10.1234km").unwrap(), Distance(10, 123));
        assert_eq!(Distance::new("10m").unwrap(), Distance(0, 10));

        assert_eq!(Distance::new("1234m").unwrap(), Distance(1, 234));
        assert_eq!(Distance::new("12345m").unwrap(), Distance(12, 345));
    }
}
