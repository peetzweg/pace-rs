use std::{fmt::Display, ops::Div};

use crate::space::{Distance, Meters};
use crate::time::{Duration, Seconds};

pub mod space;
pub mod time;

#[derive(Debug, PartialEq)]
pub struct Pace(u32, u32);

impl Display for Pace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02} min/km", self.0, self.1)
    }
}

impl<T: Into<Seconds>> Div<T> for Meters {
    type Output = Pace;

    fn div(self, rhs: T) -> Self::Output {
        let kms = self.0 / 1000;
        let secs: Seconds = rhs.into();
        let pace = secs.0 / kms;

        Pace(pace / 60, pace % 60)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().take(3).collect();

    match args.as_slice() {
        [_, distance, duration] => {
            let distance = Distance::new(distance).unwrap();
            let duration =
                Duration::try_from(duration).expect("Invalid duration, use format HH:MM:SS");

            let pace = Meters::from(distance) / Seconds::from(duration);

            println!("{}", pace);
        }
        _ => {
            eprintln!("Usage: pace <distance> <time>");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::{Duration, Hours, Minutes, Seconds};

    #[test]
    fn test_pace() {
        assert_eq!(Meters(10000) / Seconds(60 * 60), Pace(6, 0));
        assert_eq!(Meters(10000) / Minutes(60), Pace(6, 0));
        assert_eq!(Meters(10000) / Hours(1), Pace(6, 0));
        assert_eq!(Meters(10000) / Duration(1, 0, 0), Pace(6, 0));
        assert_eq!(Meters(10000) / Duration(0, 60, 0), Pace(6, 0));
        assert_eq!(Meters(10000) / Duration(0, 0, 60 * 60), Pace(6, 0));
    }
}
