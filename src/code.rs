use std::fmt;

use chrono::NaiveTime;

pub struct TimeRange {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

#[allow(dead_code)]
impl TimeRange {
    /// Creates a new `TimeRange` from the start and end times.
    ///
    /// # Parameters
    /// - `start`: The start time as a string slice in "HH:mm" format.
    /// - `end`: The end time as a string slice in "HH:mm" format.
    ///
    /// # Returns
    /// Returns a `TimeRange` instance with the specified start and end times.
    fn new<'a>(start: &'a str, end: &'a str) -> TimeRange {
        let start: Vec<u32> = start
            .split(':')
            .map(|s| s.parse::<u32>().expect("Invalid number in start time"))
            .collect();
        let end: Vec<u32> = end
            .split(':')
            .map(|s| s.parse::<u32>().expect("Invalid number in end time"))
            .collect();

        TimeRange {
            start: NaiveTime::from_hms_opt(start[0], start[1], 0).unwrap(),
            end: NaiveTime::from_hms_opt(end[0], end[1], 0).unwrap(),
        }
    }

    /// Creates a new `TimeRange` from the code.
    ///
    /// # Parameters
    /// - `code`: The code that corresponds to a TimeRange
    ///
    /// # Returns
    /// Returns a `Option<TimeRange>` with the times and returns None if it didn't find the code.
    pub fn find_code<'a>(code: &'a str) -> Option<TimeRange> {
        match code.to_uppercase().as_str() {
            "F187" => Some(TimeRange::new("04:30", "13:30")),
            "F195" => Some(TimeRange::new("04:30", "14:30")),
            "F201" => Some(TimeRange::new("04:30", "14:30")),
            "F82" => Some(TimeRange::new("04:00", "13:00")),
            "F96" => Some(TimeRange::new("04:00", "14:00")),
            "G181" => Some(TimeRange::new("05:30", "14:30")),
            "G186" => Some(TimeRange::new("05:30", "14:30")),
            "G197" => Some(TimeRange::new("05:30", "15:30")),
            "G3" => Some(TimeRange::new("05:00", "9:00")),
            "G78" => Some(TimeRange::new("05:00", "14:00")),
            "G82" => Some(TimeRange::new("05:00", "14:00")),
            "G91" => Some(TimeRange::new("05:00", "15:00")),
            "G96" => Some(TimeRange::new("05:00", "15:00")),
            "H187" => Some(TimeRange::new("06:30", "15:30")),
            "H201" => Some(TimeRange::new("06:30", "16:30")),
            "H3" => Some(TimeRange::new("06:00", "10:00")),
            "H72" => Some(TimeRange::new("06:00", "15:00")),
            "H73" => Some(TimeRange::new("06:00", "15:00")),
            "H96" => Some(TimeRange::new("06:00", "16:00")),
            "I108" => Some(TimeRange::new("07:30", "11:30")),
            "I177" => Some(TimeRange::new("07:30", "16:30")),
            "I3" => Some(TimeRange::new("07:00", "11:00")),
            "I74" => Some(TimeRange::new("07:00", "16:00")),
            "I92" => Some(TimeRange::new("07:00", "17:00")),
            "I94" => Some(TimeRange::new("07:00", "17:00")),
            "I96" => Some(TimeRange::new("07:00", "17:00")),
            "J108" => Some(TimeRange::new("08:30", "12:30")),
            "J177" => Some(TimeRange::new("08:30", "17:30")),
            "J178" => Some(TimeRange::new("08:30", "17:30")),
            "J180" => Some(TimeRange::new("08:30", "17:30")),
            "J3" => Some(TimeRange::new("08:00", "12:00")),
            "J74" => Some(TimeRange::new("08:00", "17:00")),
            "J92" => Some(TimeRange::new("08:00", "18:00")),
            "J96" => Some(TimeRange::new("08:00", "18:00")),
            "K108" => Some(TimeRange::new("09:30", "13:30")),
            "K177" => Some(TimeRange::new("09:30", "18:30")),
            "K3" => Some(TimeRange::new("09:00", "13:00")),
            "K74" => Some(TimeRange::new("09:00", "18:00")),
            "K78" => Some(TimeRange::new("09:00", "18:00")),
            "K92" => Some(TimeRange::new("09:00", "19:00")),
            "L179" => Some(TimeRange::new("10:30", "19:30")),
            "L194" => Some(TimeRange::new("10:30", "20:30")),
            "L3" => Some(TimeRange::new("10:00", "14:00")),
            "M110" => Some(TimeRange::new("11:30", "15:30")),
            "M181" => Some(TimeRange::new("11:30", "20:30")),
            "M4" => Some(TimeRange::new("11:00", "15:00")),
            "M75" => Some(TimeRange::new("11:00", "20:00")),
            "M90" => Some(TimeRange::new("11:00", "21:00")),
            "N188" => Some(TimeRange::new("12:30", "21:30")),
            "N190" => Some(TimeRange::new("12:30", "21:30")),
            "N80" => Some(TimeRange::new("12:00", "21:00")),
            "O20" => Some(TimeRange::new("13:00", "17:00")),
            "O95" => Some(TimeRange::new("13:00", "22:00")),
            "O99" => Some(TimeRange::new("13:00", "22:00")),
            "P37" => Some(TimeRange::new("14:00", "18:00")),
            "Q165" => Some(TimeRange::new("15:30", "01:30")),
            "Q179" => Some(TimeRange::new("15:30", "19:30")),
            "Q31" => Some(TimeRange::new("15:00", "00:00")),
            "R202" => Some(TimeRange::new("16:30", "20:30")),
            "R90" => Some(TimeRange::new("16:00", "20:00")),
            "S104" => Some(TimeRange::new("17:00", "21:00")),
            "S209" => Some(TimeRange::new("17:30", "21:30")),
            "T105" => Some(TimeRange::new("18:00", "22:00")),
            "T77" => Some(TimeRange::new("18:00", "03:00")),
            "V181" => Some(TimeRange::new("20:30", "05:30")),
            "W3" => Some(TimeRange::new("21:00", "01:00")),
            "W74" => Some(TimeRange::new("21:00", "06:00")),
            _ => None,
        }
    }
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}",
            self.start.format("%H:%M"),
            self.end.format("%H:%M")
        )
    }
}

#[allow(dead_code)]
pub enum MonthYear {
    Month,
    Year,
}

impl MonthYear {
    #[allow(dead_code)]
    pub fn validate(&self, val: u32) -> Result<u32, ()> {
        match *self {
            MonthYear::Month => {
                if (1..=12).contains(&val) {
                    Ok(val)
                } else {
                    Err(())
                }
            }
            MonthYear::Year => {
                if (2000..=3000).contains(&val) {
                    Ok(val)
                } else {
                    Err(())
                }
            }
        }
    }
}

impl fmt::Display for MonthYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MonthYear::Year => "year",
                MonthYear::Month => "month",
            }
        )
    }
}
