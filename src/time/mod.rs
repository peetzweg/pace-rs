use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Seconds(pub u32);

pub struct Minutes(pub u32);

pub struct Hours(pub u32);

#[derive(Debug, PartialEq)]
pub struct Duration(pub u32, pub u32, pub u32);

impl TryFrom<&String> for Duration {
    type Error = bool;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let v = value.split(":").take(3).collect::<Vec<&str>>();
        match v.as_slice() {
            [h, m, s] => {
                let h = h.parse::<u32>().unwrap();
                let m = m.parse::<u32>().unwrap().clamp(0, 59);
                let s = s.parse::<u32>().unwrap().clamp(0, 59);

                Ok(Self(h, m, s))
            }
            [m, s] => {
                let m = m.parse::<u32>().unwrap_or(0);
                let s = s
                    .parse::<u32>()
                    .expect("'ss' in format 'MM:ss' is not a valid number")
                    .clamp(0, 59);

                Ok(Self(0, m, s))
            }
            [s] => {
                let s = s.parse::<u32>().unwrap_or(0);

                Ok(Self(0, 0, s))
            }
            _ => Err(false),
        }
    }
}

impl From<Hours> for Seconds {
    fn from(hours: Hours) -> Self {
        Seconds(hours.0 * 3600)
    }
}

impl From<Minutes> for Seconds {
    fn from(minutes: Minutes) -> Self {
        Seconds(minutes.0 * 60)
    }
}

impl From<Duration> for Seconds {
    fn from(duration: Duration) -> Self {
        let hours: Seconds = Hours(duration.0).into();
        let minutes: Seconds = Minutes(duration.1).into();
        let seconds: Seconds = Seconds(duration.2).into();

        Seconds(hours.0 + minutes.0 + seconds.0)
    }
}

impl From<Seconds> for Duration {
    fn from(seconds: Seconds) -> Self {
        let hours = seconds.0 / 3600;
        let minutes = (seconds.0 % 3600) / 60;
        let seconds = seconds.0 % 60;

        Duration(hours, minutes, seconds)
    }
}

impl From<Minutes> for Duration {
    fn from(minutes: Minutes) -> Self {
        let seconds: Seconds = minutes.into();

        seconds.into()
    }
}
impl From<Hours> for Duration {
    fn from(minutes: Hours) -> Self {
        let seconds: Seconds = minutes.into();

        seconds.into()
    }
}

impl Display for Minutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} min", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds_from() {
        assert_eq!(Seconds::from(Minutes(1)), Seconds(60));
        assert_eq!(Seconds::from(Hours(1)), Seconds(60 * 60));
        assert_eq!(Seconds::from(Duration(1, 1, 1)), Seconds(60 * 60 + 60 + 1));
    }

    #[test]
    fn duration_from() {
        assert_eq!(Duration::from(Seconds(128)), Duration(0, 2, 8));
        assert_eq!(Duration::from(Hours(2)), Duration(2, 0, 0));
        assert_eq!(Duration::from(Minutes(128)), Duration(2, 8, 0));

        assert_eq!(
            Duration::try_from(&String::from("12:34:56")).unwrap(),
            Duration(12, 34, 56)
        );
        assert_eq!(
            Duration::try_from(&String::from("34:56")).unwrap(),
            Duration(0, 34, 56)
        );

        assert_eq!(
            Duration::try_from(&String::from("56")).unwrap(),
            Duration(0, 0, 56)
        );
    }
}
