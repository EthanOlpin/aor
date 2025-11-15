use anyhow::anyhow;
use chrono::TimeZone;
use chrono::{DateTime, Datelike, Local};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EventDate {
    pub day: u8,
    pub year: u16,
}

impl Default for EventDate {
    fn default() -> Self {
        let current_date = Local::now().with_timezone(&chrono_tz::EST);
        if current_date.month() == 12 {
            EventDate {
                day: current_date.day() as u8,
                year: current_date.year() as u16,
            }
        } else {
            EventDate {
                day: 1,
                year: current_date.year() as u16,
            }
        }
    }
}

impl EventDate {
    pub fn default_or(day: Option<u8>, year: Option<u16>) -> Self {
        let mut date = EventDate::default();
        if let Some(day) = day {
            date.day = day;
        }
        if let Some(year) = year {
            date.year = year;
        }
        date
    }
}

impl TryInto<DateTime<chrono_tz::Tz>> for EventDate {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<DateTime<chrono_tz::Tz>, Self::Error> {
        chrono_tz::EST
            .with_ymd_and_hms(self.year.into(), 12, self.day.into(), 0, 0, 0)
            .earliest()
            .ok_or(anyhow!("Not a valid date 12/{}/{}", self.day, self.year))
    }
}
