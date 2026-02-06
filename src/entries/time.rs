use std::fmt::Display;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct LogDate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct LogTime {
    pub date: Option<LogDate>,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: Option<u16>,
}

impl LogTime {
    pub fn parse(time_str: &str) -> Option<Self> {
        if let Some(time) = Self::parse_rfc_3339(time_str) {
            return Some(time);
        }
        let (without_suffix, _is_pm) = strip_meridiem(time_str);
        let (date, time_part) = if let Some((date_str, time_str)) = without_suffix.split_once(' ') {
            (Some(LogDate::parse(date_str)?), time_str)
        }
        else {
            (None, without_suffix)
        };
        let (hour, minute, second, millisecond) = parse_time(time_part)?;
        let time = LogTime {
            date,
            hour,
            minute,
            second,
            millisecond,
        };
        Some(time)
    }

    fn parse_rfc_3339(time_str: &str) -> Option<Self> {
        // e.g. "2025-10-30T19:21:06.036061Z"
        let (date_str, time_str) = time_str.strip_suffix('Z')?.split_once('T')?;
        let date = parse_iso_date(date_str)?;
        let (hour, minute, second, millisecond) = parse_iso_time(time_str)?;
        Some(LogTime {
            date: Some(date),
            hour,
            minute,
            second,
            millisecond: Some(millisecond),
        })
    }
}

impl Display for LogTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(date) = &self.date {
            write!(f, "{:04}-{:02}-{:02} ", date.year, date.month, date.day)?;
        }
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)?;
        if let Some(ms) = &self.millisecond {
            write!(f, ".{:03}", ms)?;
        }

        Ok(())
    }
} 

// "17:45:36.659"
fn parse_time(time: &str) -> Option<(u8, u8, u8, Option<u16>)> {
    let (hms_part, millisecond) = if let Some((hms_str, ms_str)) = time.split_once('.') {
        (hms_str, Some(ms_str.parse::<u16>().ok()?))
    }
    else {
        (time, None)
    };
    let (hour_str, minute_and_second) = hms_part.split_once(':')?;
    let (minute_str, second_str) = minute_and_second.split_once(':')?;
    let hour: u8 = hour_str.parse().ok()?;
    let minute: u8 = minute_str.parse().ok()?;
    let second: u8 = second_str.parse().ok()?;
    Some((hour, minute, second, millisecond))
}

// "19:21:06.036061" -> {19, 21, 6, 36}
fn parse_iso_time(time: &str) -> Option<(u8, u8, u8, u16)> {
    let hour: u8 = time.get(0..2)?.parse().ok()?;
    if time.get(2..3)? != ":" { return None; }
    let minute: u8 = time.get(3..5)?.parse().ok()?;
    if time.get(5..6)? != ":" { return None; }
    let second: u8 = time.get(6..8)?.parse().ok()?;
    if time.get(8..9)? != "." { return None; }
    let millisecond: u16 = time.get(9..12)?.parse().ok()?;
    Some((hour, minute, second, millisecond))
}

// "2025-10-30" -> {2025, 10, 30}
fn parse_iso_date(date: &str) -> Option<LogDate> {
    let year: u16 = date.get(0..4)?.parse().ok()?;
    if date.get(4..5)? != "-" { return None; }
    let month: u8 = date.get(5..7)?.parse().ok()?;
    if date.get(7..8)? != "-" { return None; }
    let day: u8 = date.get(8..10)?.parse().ok()?;
    Some(LogDate { day, month, year })
}

// "02/12/2025" -> {2025, 12, 2}
fn parse_eu_date(date: &str) -> Option<LogDate> {
    let (day_str, month_and_year) = date.split_once('/')?;
    let (month_str, year_str) = month_and_year.split_once('/')?;
    let day: u8 = day_str.parse().ok()?;
    let month: u8 = month_str.parse().ok()?;
    let year: u16 = year_str.parse().ok()?;
    Some(LogDate { day, month, year })
}

// "04Dec2025" -> {2025, 12, 04}
fn parse_named_month_date(date: &str) -> Option<LogDate> {
    if date.len() < 9 {
        return None;
    }

    // e.g. "04Dec2025", "16Sept2025"
    let day: u8 = date.get(0..2)?.parse().ok()?;
    let date_and_month_str = date.get(2..)?;
    let mut iter = date_and_month_str.chars().peekable();
    let mut month_name = String::new();
    let mut year: u16 = 0;
    while let Some(c) = iter.peek() && c.is_alphabetic()
    {
        // Collect "Dec" or "Sept"
        month_name.push(*c);
        iter.next();
    }
    while let Some(c) = iter.next() {
        year = year * 10 + (c.to_digit(10)? as u16);
    }
    let month: u8 = month_str_to_number(&month_name)?;
    Some(LogDate { day, month, year })
}

/// Strips " AM" or " PM" suffix from the time string and indicates if it was PM.
fn strip_meridiem(time_str: &str) -> (&str, bool) {
    if let Some(stripped) = time_str.strip_suffix(" AM") {
        (stripped, false)
    } else if let Some(stripped) = time_str.strip_suffix(" PM") {
        (stripped, true)
    } else {
        (time_str, false)
    }
}

/// O(1) parses date strings in various formats
impl LogDate {
    fn parse(date_str: &str) -> Option<LogDate> {
        if date_str.len() > 23 {
            // "12Sept2025 00:41:16.572"
            return None;
        }
        else if let Some(date) = parse_iso_date(date_str) {
            Some(date)
        }
        else if let Some(date) = parse_eu_date(date_str) {
            Some(date)
        }
        else if let Some(date) = parse_named_month_date(date_str) {
            Some(date)
        }
        else {
            None
        }
    }
}

fn month_str_to_number(month_str: &str) -> Option<u8> {
    // TODO expand to support more variations once more reference logs are collected
    match month_str {
        "Jan" => Some(1),
        "Feb" => Some(2),
        "Mar" => Some(3),
        "Apr" => Some(4),
        "May" => Some(5),
        "Jun" => Some(6),
        "Jul" => Some(7),
        "Aug" => Some(8),
        "Sep" => Some(9),
        "Sept" => Some(9),
        "Oct" => Some(10),
        "Nov" => Some(11),
        "Dec" => Some(12),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        let time_str = "16:20:50";
        let time = LogTime::parse(time_str).expect("Failed to parse simple time");
        assert!(time.date.is_none());
        assert_eq!(time.hour, 16);
        assert_eq!(time.minute, 20);
        assert_eq!(time.second, 50);
        assert!(time.millisecond.is_none());
    }

    #[test]
    fn date() {
        let time_str = "02/12/2025 14:45:51 PM";
        let time = LogTime::parse(time_str).expect("Failed to parse date and time");
        assert_eq!(time.date.map(|d| (d.day, d.month, d.year)), Some((2, 12, 2025)));
        assert_eq!(time.hour, 14);
        assert_eq!(time.minute, 45);
        assert_eq!(time.second, 51);
    }

    #[test]
    fn milliseconds() {
        let time_str = "08:33:03.471";
        let time = LogTime::parse(time_str).expect("Failed to parse time with milliseconds");
        assert!(time.date.is_none());
        assert_eq!(time.hour, 8);
        assert_eq!(time.minute, 33);
        assert_eq!(time.second, 3);
        assert_eq!(time.millisecond, Some(471));
    }

    #[test]
    fn named_month() {
        let time_str = "04Dec2025 20:16:35.371";
        let time = LogTime::parse(time_str).expect("Failed to parse time with named month");
        assert_eq!(time.date.map(|d| (d.day, d.month, d.year)), Some((4, 12, 2025)));
        assert_eq!(time.hour, 20);
        assert_eq!(time.minute, 16);
        assert_eq!(time.second, 35);
        assert_eq!(time.millisecond, Some(371));
    }

    #[test]
    fn rfc3339() {
        let time_str = "2025-10-30T19:21:06.036061Z";
        let time = LogTime::parse(time_str).expect("Failed to parse RFC3339 time");
        assert_eq!(time.date.map(|d| (d.day, d.month, d.year)), Some((30, 10, 2025)));
        assert_eq!(time.hour, 19);
        assert_eq!(time.minute, 21);
        assert_eq!(time.second, 06);
        assert_eq!(time.millisecond, Some(036));
    }

    #[test]
    fn various() {
        let examples = vec![
            "01:53:30",
            "01:54:42",
            "01:56:17",
            "02/12/2025 14:45:51 PM",
            "02/12/2025 15:05:28 PM",
            "02/12/2025 15:05:29 PM",
            "04Aug2025 17:45:36.659",
            "04Aug2025 17:45:36.946",
            "04Dec2025 20:16:35.371",
            "04Dec2025 20:16:36.602",
            "24Sep2025 10:54:01.697",
            "16Sept2025 23:44:52.173", // <-- Yes, that t isn't a typo
            "12Oct2025 00:41:16.062",
            "12Oct2025 00:41:16.572",
            "12Sept2025 00:41:16.572",
            "2025-10-30T19:21:06.036061Z",
            "08:01:29.968",
            "08:33:03.106",
            "08:33:03.471",
            "2024-07-11 04:30:53",
        ];
        for time_str in examples {
            let _time = LogTime::parse(time_str).expect(&format!("Failed to parse time: {}", time_str));
        }
    }
}
