#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LogPrefixDate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LogPrefixTime {
    pub date: Option<LogPrefixDate>,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: Option<u16>,
}

impl LogPrefixTime {
    pub fn parse(time_str: &str) -> Option<LogPrefixTime> {
        let (without_suffix, _is_pm) = strip_meridiem(time_str);
        let space_index = without_suffix.find(' ');
        let (date, time_part) = if let Some(index) = space_index {
            let date_str = without_suffix.get(..index)?;
            let time_str = without_suffix.get(index + 1..)?;
            (Some(parse_date(date_str)?), time_str)
        } else {
            (None, without_suffix)
        };
        let (hms_part, millisecond) = if let Some(dot_index) = time_part.find('.') {
            (time_part.get(..dot_index)?, Some(time_part.get(dot_index + 1..)?.parse::<u16>().ok()?))
        } else {
            (time_part, None)
        };
        let (hour_str, rest) = hms_part.split_once(':')?;
        let (minute_str, second_str) = rest.split_once(':')?;
        let hour: u8 = hour_str.parse().ok()?;
        let minute: u8 = minute_str.parse().ok()?;
        let second: u8 = second_str.parse().ok()?;
        let time = LogPrefixTime {
            date,
            hour,
            minute,
            second,
            millisecond,
        };
        Some(time)
    }
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
fn parse_date(date_str: &str) -> Option<LogPrefixDate> {
    if date_str.len() > 23 { // "12Sept2025 00:41:16.572"
        return None;
    }
    if let Some((day_str, rest)) = date_str.split_once('/') { // e.g. "02/12/2025"
        let (month_str, year_str) = rest.split_once('/')?;
        let day: u8 = day_str.parse().ok()?;
        let month: u8 = month_str.parse().ok()?;
        let year: u16 = year_str.parse().ok()?;
        Some(LogPrefixDate { day, month, year })
    } else if date_str.len() >= 9 { // e.g. "04Dec2025", "16Sept2025"
        let day: u8 = date_str.get(0..2)?.parse().ok()?;
        let date_and_month_str = date_str.get(2..)?;
        let mut iter = date_and_month_str.chars().peekable();
        let mut month_name = String::new();
        let mut year: u16 = 0;
        while let Some(c) = iter.peek() && c.is_alphabetic() { // Collect "Dec" or "Sept"
            month_name.push(*c);
            iter.next();
        }
        while let Some(c) = iter.next() {
            year = year * 10 + (c.to_digit(10)? as u16);
        }
        let month: u8 = month_str_to_number(&month_name)?;
        Some(LogPrefixDate { day, month, year })
    } else {
        None
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
    fn test_parse_simple_time() {
        let time_str = "16:20:50";
        let time = LogPrefixTime::parse(time_str).expect("Failed to parse simple time");
        assert!(time.date.is_none());
        assert_eq!(time.hour, 16);
        assert_eq!(time.minute, 20);
        assert_eq!(time.second, 50);
        assert!(time.millisecond.is_none());
    }

    #[test]
    fn test_parse_date_and_time() {
        let time_str = "02/12/2025 14:45:51 PM";
        let time = LogPrefixTime::parse(time_str).expect("Failed to parse date and time");
        assert_eq!(time.date.map(|d| (d.day, d.month, d.year)), Some((2, 12, 2025)));
        assert_eq!(time.hour, 14);
        assert_eq!(time.minute, 45);
        assert_eq!(time.second, 51);
    }

    #[test]
    fn test_parse_time_with_milliseconds() {
        let time_str = "08:33:03.471";
        let time = LogPrefixTime::parse(time_str).expect("Failed to parse time with milliseconds");
        assert!(time.date.is_none());
        assert_eq!(time.hour, 8);
        assert_eq!(time.minute, 33);
        assert_eq!(time.second, 3);
        assert_eq!(time.millisecond, Some(471));
    }

    #[test]
    fn test_parse_time_with_named_month() {
        let time_str = "04Dec2025 20:16:35.371";
        let time = LogPrefixTime::parse(time_str).expect("Failed to parse time with named month");
        assert_eq!(time.date.map(|d| (d.day, d.month, d.year)), Some((4, 12, 2025)));
        assert_eq!(time.hour, 20);
        assert_eq!(time.minute, 16);
        assert_eq!(time.second, 35);
        assert_eq!(time.millisecond, Some(371));
    }

    #[test]
    fn test_parse_various_formats() {
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
            "08:01:29.968",
            "08:33:03.106",
            "08:33:03.471",
        ];
        for time_str in examples {
            let _time = LogPrefixTime::parse(time_str).expect(&format!("Failed to parse time: {}", time_str));
        }
    }
}
