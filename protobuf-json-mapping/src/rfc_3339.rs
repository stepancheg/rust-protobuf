use std::fmt;
use std::time::Duration;

// Number of seconds in a day is a constant.
// We do not support leap seconds here.
const SECONDS_IN_DAY: u64 = 86400;

// Gregorian calendar has 400 years cycles, this is a procedure
// for computing if a year is a leap year.
fn is_leap_year(year: i64) -> bool {
    if year % 4 != 0 {
        false
    } else if year % 100 != 0 {
        true
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}

fn days_in_year(year: i64) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

// Number of leap years among 400 consecutive years.
const CYCLE_LEAP_YEARS: u32 = 400 / 4 - 400 / 100 + 400 / 400;
// Number of days in 400 years cycle.
const CYCLE_DAYS: u32 = 400 * 365 + CYCLE_LEAP_YEARS;
// Number of seconds in 400 years cycle.
const CYCLE_SECONDS: u64 = CYCLE_DAYS as u64 * SECONDS_IN_DAY;

// Number of seconds between 1 Jan 1970 and 1 Jan 2000.
// Check with:
// `TZ=UTC gdate --rfc-3339=seconds --date @946684800`
const YEARS_1970_2000_SECONDS: u64 = 946684800;
// Number of seconds between 1 Jan 1600 and 1 Jan 1970.
const YEARS_1600_1970_SECONDS: u64 = CYCLE_SECONDS - YEARS_1970_2000_SECONDS;

// For each year in the cycle, number of leap years before in the cycle.
#[cfg_attr(rustfmt, rustfmt_skip)]
static YEAR_DELTAS: [u8; 401] = [
    0,  1,  1,  1,  1,  2,  2,  2,  2,  3,  3,  3,  3,  4,  4,  4,  4,  5,  5,  5,
    5,  6,  6,  6,  6,  7,  7,  7,  7,  8,  8,  8,  8,  9,  9,  9,  9, 10, 10, 10,
    10, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14, 14, 15, 15, 15,
    15, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20, 20,
    20, 21, 21, 21, 21, 22, 22, 22, 22, 23, 23, 23, 23, 24, 24, 24, 24, 25, 25, 25, // 100
    25, 25, 25, 25, 25, 26, 26, 26, 26, 27, 27, 27, 27, 28, 28, 28, 28, 29, 29, 29,
    29, 30, 30, 30, 30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33, 34, 34, 34,
    34, 35, 35, 35, 35, 36, 36, 36, 36, 37, 37, 37, 37, 38, 38, 38, 38, 39, 39, 39,
    39, 40, 40, 40, 40, 41, 41, 41, 41, 42, 42, 42, 42, 43, 43, 43, 43, 44, 44, 44,
    44, 45, 45, 45, 45, 46, 46, 46, 46, 47, 47, 47, 47, 48, 48, 48, 48, 49, 49, 49, // 200
    49, 49, 49, 49, 49, 50, 50, 50, 50, 51, 51, 51, 51, 52, 52, 52, 52, 53, 53, 53,
    53, 54, 54, 54, 54, 55, 55, 55, 55, 56, 56, 56, 56, 57, 57, 57, 57, 58, 58, 58,
    58, 59, 59, 59, 59, 60, 60, 60, 60, 61, 61, 61, 61, 62, 62, 62, 62, 63, 63, 63,
    63, 64, 64, 64, 64, 65, 65, 65, 65, 66, 66, 66, 66, 67, 67, 67, 67, 68, 68, 68,
    68, 69, 69, 69, 69, 70, 70, 70, 70, 71, 71, 71, 71, 72, 72, 72, 72, 73, 73, 73, // 300
    73, 73, 73, 73, 73, 74, 74, 74, 74, 75, 75, 75, 75, 76, 76, 76, 76, 77, 77, 77,
    77, 78, 78, 78, 78, 79, 79, 79, 79, 80, 80, 80, 80, 81, 81, 81, 81, 82, 82, 82,
    82, 83, 83, 83, 83, 84, 84, 84, 84, 85, 85, 85, 85, 86, 86, 86, 86, 87, 87, 87,
    87, 88, 88, 88, 88, 89, 89, 89, 89, 90, 90, 90, 90, 91, 91, 91, 91, 92, 92, 92,
    92, 93, 93, 93, 93, 94, 94, 94, 94, 95, 95, 95, 95, 96, 96, 96, 96, 97, 97, 97, 97,
];

/// UTC time
pub struct TmUtc {
    /// Year
    year: i64,
    /// 1..=12
    month: u32,
    /// 1-based day of month
    day: u32,
    /// 0..=23
    hour: u32,
    /// 0..=59
    minute: u32,
    /// 0..=59; no leap seconds
    second: u32,
    /// 0..=999_999_999
    nanos: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum Rfc3339ParseError {
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("Trailing characters")]
    TrailngCharacters,
    #[error("Expecting digits")]
    ExpectingDigits,
    #[error("Expecting character: {:?}", .0)]
    ExpectingChar(char),
    #[error("Expecting timezone")]
    ExpectingTimezone,
    #[error("No digits after dot")]
    NoDigitsAfterDot,
    #[error("Date-time field is out of range")]
    DateTimeFieldOutOfRange,
    #[error("Expecting date-time separator")]
    ExpectingDateTimeSeparator,
}

pub type Rfc3339ParseResult<A> = Result<A, Rfc3339ParseError>;

impl TmUtc {
    fn day_of_cycle_to_year_day_of_year(day_of_cycle: u32) -> (i64, u32) {
        debug_assert!(day_of_cycle < CYCLE_DAYS);

        let mut year_mod_400 = (day_of_cycle / 365) as i64;
        let mut day_or_year = (day_of_cycle % 365) as u32;

        let delta = YEAR_DELTAS[year_mod_400 as usize] as u32;
        if day_or_year < delta {
            year_mod_400 -= 1;
            day_or_year += 365 - YEAR_DELTAS[year_mod_400 as usize] as u32;
        } else {
            day_or_year -= delta;
        }

        (year_mod_400, day_or_year)
    }

    fn year_day_of_year_to_day_of_cycle(year_mod_400: u32, day_of_year: u32) -> u32 {
        debug_assert!(year_mod_400 < 400);
        debug_assert!(day_of_year < days_in_year(year_mod_400 as i64));

        year_mod_400 * 365 + YEAR_DELTAS[year_mod_400 as usize] as u32 + day_of_year
    }

    // Convert seconds of the day of hour, minute and second
    fn second_of_day_to_h_m_s(seconds: u32) -> (u32, u32, u32) {
        debug_assert!(seconds < 86400);

        let hour = seconds / 3600;
        let minute = seconds % 3600 / 60;
        let second = seconds % 60;

        (hour, minute, second)
    }

    fn h_m_s_to_second_of_day(hour: u32, minute: u32, second: u32) -> u32 {
        debug_assert!(hour < 24);
        debug_assert!(minute < 60);
        debug_assert!(second < 60);

        hour * 3600 + minute * 60 + second
    }

    fn days_in_months(year: i64) -> &'static [u32] {
        if is_leap_year(year) {
            &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        }
    }

    // Convert day of year (0-based) to month and day
    fn day_of_year_to_month_day(year: i64, day_of_year: u32) -> (u32, u32) {
        debug_assert!(day_of_year < days_in_year(year));

        let days_in_months = TmUtc::days_in_months(year);

        let mut rem_days = day_of_year;
        let mut month = 1;
        while rem_days >= days_in_months[month - 1] {
            rem_days -= days_in_months[month - 1];
            month += 1;
        }

        debug_assert!(rem_days + 1 <= days_in_months[month - 1]);

        (month as u32, rem_days + 1)
    }

    fn month_day_to_day_of_year(year: i64, month: u32, day: u32) -> u32 {
        debug_assert!(month >= 1);
        debug_assert!(month <= 12);

        debug_assert!(day >= 1);

        let days_in_months = TmUtc::days_in_months(year);

        // TODO: replace loop with precomputed table
        let mut day_of_year = 0;
        for next_month in 1..month {
            day_of_year += days_in_months[next_month as usize - 1];
        }

        debug_assert!(day <= days_in_months[month as usize - 1]);

        day_of_year + day - 1
    }

    // Construct from duration added to cycle start year
    fn from_cycle_start_add_duration(mut cycle_start: i64, add: Duration) -> TmUtc {
        debug_assert!(cycle_start % 400 == 0);

        // Split duration to days and duration within day

        let days = add.as_secs() / SECONDS_IN_DAY;
        let duration_of_day = add - Duration::from_secs(days * SECONDS_IN_DAY);

        let cycles = days / CYCLE_DAYS as u64;
        cycle_start += cycles as i64 * 400;
        let day_of_cycle = days % CYCLE_DAYS as u64;

        let (year_mod_400, day_of_year) =
            TmUtc::day_of_cycle_to_year_day_of_year(day_of_cycle as u32);

        let (year,) = (cycle_start + year_mod_400,);
        let (month, day) = TmUtc::day_of_year_to_month_day(year, day_of_year);
        let (hour, minute, second) =
            TmUtc::second_of_day_to_h_m_s(duration_of_day.as_secs() as u32);

        TmUtc {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanos: duration_of_day.subsec_nanos(),
        }
    }

    // Protobuf timestamp: seconds from epoch, and nanos 0..=999_999_999 counting forward.
    pub fn from_protobuf_timestamp(seconds: i64, nanos: u32) -> TmUtc {
        assert!(nanos <= 999_999_999);

        let (mut year, mut seconds) = if seconds >= 0 {
            (1970, seconds as u64)
        } else {
            let minus_seconds = if seconds == i64::MIN {
                i64::MIN as u64
            } else {
                -seconds as u64
            };

            let cycles = (minus_seconds + CYCLE_SECONDS) / CYCLE_SECONDS;

            (
                1970 - 400 * cycles as i64,
                cycles * CYCLE_SECONDS - minus_seconds,
            )
        };

        year -= 370;
        seconds += YEARS_1600_1970_SECONDS;

        TmUtc::from_cycle_start_add_duration(year, Duration::new(seconds, nanos))
    }

    pub fn to_protobuf_timestamp(&self) -> (i64, u32) {
        assert!(self.year >= 0);
        assert!(self.year <= 9999);

        let year_mod_400 = ((self.year % 400 + 400) % 400) as u32;
        let cycle_start = self.year - year_mod_400 as i64;

        let day_of_year = TmUtc::month_day_to_day_of_year(self.year, self.month, self.day);
        let day_of_cycle = TmUtc::year_day_of_year_to_day_of_cycle(year_mod_400, day_of_year);
        let second_of_day = TmUtc::h_m_s_to_second_of_day(self.hour, self.minute, self.second);

        let second_of_cycle = day_of_cycle as u64 * SECONDS_IN_DAY + second_of_day as u64;

        let epoch_seconds = (cycle_start - 1600) / 400 * CYCLE_SECONDS as i64
            - YEARS_1600_1970_SECONDS as i64
            + second_of_cycle as i64;

        (epoch_seconds, self.nanos)
    }

    pub fn parse_rfc_3339(s: &str) -> Rfc3339ParseResult<(i64, u32)> {
        struct Parser<'a> {
            s: &'a [u8],
            pos: usize,
        }

        impl<'a> Parser<'a> {
            fn next_number(&mut self, len: usize) -> Rfc3339ParseResult<u32> {
                let end_pos = self.pos + len;
                if end_pos > self.s.len() {
                    return Err(Rfc3339ParseError::UnexpectedEof);
                }
                let mut r = 0;
                for i in 0..len {
                    let c = self.s[self.pos + i];
                    if c >= b'0' && c <= b'9' {
                        r = r * 10 + (c - b'0') as u32;
                    } else {
                        return Err(Rfc3339ParseError::ExpectingDigits);
                    }
                }
                self.pos += len;
                Ok(r)
            }

            fn lookahead_char(&self) -> Rfc3339ParseResult<u8> {
                if self.pos == self.s.len() {
                    return Err(Rfc3339ParseError::UnexpectedEof);
                }
                Ok(self.s[self.pos])
            }

            fn next_char(&mut self, expect: u8) -> Rfc3339ParseResult<()> {
                assert!(expect < 0x80);
                let c = self.lookahead_char()?;
                if c != expect {
                    return Err(Rfc3339ParseError::ExpectingChar(expect as char));
                }
                self.pos += 1;
                Ok(())
            }
        }

        let mut parser = Parser {
            s: s.as_bytes(),
            pos: 0,
        };

        let year = parser.next_number(4)? as i64;
        parser.next_char(b'-')?;
        let month = parser.next_number(2)?;
        parser.next_char(b'-')?;
        let day = parser.next_number(2)?;

        if month < 1 || month > 12 {
            return Err(Rfc3339ParseError::DateTimeFieldOutOfRange);
        }

        if day < 1 || day > TmUtc::days_in_months(year as i64)[month as usize - 1] {
            return Err(Rfc3339ParseError::DateTimeFieldOutOfRange);
        }

        match parser.lookahead_char()? {
            b'T' | b't' | b' ' => parser.pos += 1,
            _ => return Err(Rfc3339ParseError::ExpectingDateTimeSeparator),
        }

        let hour = parser.next_number(2)?;
        parser.next_char(b':')?;
        let minute = parser.next_number(2)?;
        parser.next_char(b':')?;
        let second = parser.next_number(2)?;

        if hour > 23 || minute > 59 || second > 60 {
            return Err(Rfc3339ParseError::DateTimeFieldOutOfRange);
        }

        // round down leap second
        let second = if second == 60 { 59 } else { second };

        let nanos = if parser.lookahead_char()? == b'.' {
            parser.pos += 1;
            let mut digits = 0;
            let mut nanos = 0;
            while parser.lookahead_char()? >= b'0' && parser.lookahead_char()? <= b'9' {
                let digit = (parser.lookahead_char()? - b'0') as u32;
                parser.pos += 1;
                if digits == 9 {
                    continue;
                }
                digits += 1;
                nanos = nanos * 10 + digit;
            }

            if digits == 0 {
                return Err(Rfc3339ParseError::NoDigitsAfterDot);
            }

            for _ in digits..9 {
                nanos *= 10;
            }
            nanos
        } else {
            0
        };

        let offset_seconds = if parser.lookahead_char()? == b'Z' || parser.lookahead_char()? == b'z'
        {
            parser.pos += 1;
            0
        } else {
            let sign = if parser.lookahead_char()? == b'+' {
                1
            } else if parser.lookahead_char()? == b'-' {
                -1
            } else {
                return Err(Rfc3339ParseError::ExpectingTimezone);
            };

            parser.pos += 1;

            let hour_offset = parser.next_number(2)?;
            parser.next_char(b':')?;
            let minute_offset = parser.next_number(2)?;

            (hour_offset * 3600 + 60 * minute_offset) as i64 * sign
        };

        if parser.pos != parser.s.len() {
            return Err(Rfc3339ParseError::TrailngCharacters);
        }

        let (seconds, nanos) = TmUtc {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanos,
        }
        .to_protobuf_timestamp();

        Ok((seconds - offset_seconds, nanos))
    }
}

impl fmt::Display for TmUtc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.year > 9999 {
            write!(f, "+{}", self.year)?;
        } else if self.year < 0 {
            write!(f, "{:05}", self.year)?;
        } else {
            write!(f, "{:04}", self.year)?;
        }
        write!(
            f,
            "-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.month, self.day, self.hour, self.minute, self.second
        )?;

        // if precision is not specified, print nanoseconds
        let subsec_digits = f.precision().unwrap_or(9);
        if subsec_digits != 0 {
            let mut subsec_digits = subsec_digits;

            let width = if subsec_digits > 9 { 9 } else { subsec_digits };

            // "Truncated" nanonseconds.
            let mut subsec = self.nanos;

            // Performs 8 iterations when precision=1,
            // but that's probably not a issue compared to other computations.
            for _ in width..9 {
                subsec /= 10;
            }

            write!(f, ".{:0width$}", subsec, width = width as usize)?;

            // Adding more than 9 digits is meaningless,
            // but if user requests it, we should print zeros.
            for _ in 9..subsec_digits {
                write!(f, "0")?;
                subsec_digits -= 1;
            }
        }

        write!(f, "Z")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fmt() {
        fn test_impl(expected: &str, secs: i64, nanos: u32, subsec_digits: u32) {
            let tm_utc = TmUtc::from_protobuf_timestamp(secs, nanos);

            assert_eq!(
                expected,
                format!("{:.prec$}", tm_utc, prec = subsec_digits as usize)
            );
        }

        // Tests can be validated with with GNU date:
        // `TZ=UTC gdate --date @1535585179 --iso-8601=seconds`

        test_impl("1970-01-01T00:00:00Z", 0, 0, 0);
        test_impl("2018-08-29T23:26:19Z", 1535585179, 0, 0);
        test_impl("2018-08-29T23:26:19.123Z", 1535585179, 123456789, 3);
        test_impl("1646-04-01T03:45:44Z", -10216613656, 0, 0);
        test_impl("1970-01-01T00:00:00.000000001000Z", 0, 1, 12);
        test_impl("5138-11-16T09:46:40Z", 100000000000, 0, 0);
        test_impl("+33658-09-27T01:46:41Z", 1000000000001, 0, 0);
        // Leading zero
        test_impl("0000-12-31T00:00:00Z", -62135683200, 0, 0);
        // Minus zero
        test_impl("-0003-10-30T14:13:20Z", -62235683200, 0, 0);
        // More than 4 digits
        // Largest value GNU date can handle
        test_impl("+2147485547-12-31T23:59:59Z", 67768036191676799, 0, 0);
        // Negative dates
        test_impl("1969-12-31T23:59:59Z", -1, 0, 0);
        test_impl("1969-12-31T23:59:00Z", -60, 0, 0);
        test_impl("1969-12-31T23:59:58.900Z", -2, 900_000_000, 3);
        test_impl("1966-10-31T14:13:20Z", -100000000, 0, 0);
        test_impl("-29719-04-05T22:13:19Z", -1000000000001, 0, 0);
        // Smallest value GNU date can handle
        test_impl("-2147481748-01-01T00:00:00Z", -67768040609740800, 0, 0);
    }

    #[test]
    fn test_parse_fmt() {
        fn test_impl(s: &str, width: usize) {
            let (seconds, nanos) = TmUtc::parse_rfc_3339(s).unwrap();
            let formatted = format!(
                "{:.width$}",
                TmUtc::from_protobuf_timestamp(seconds, nanos),
                width = width
            );
            assert_eq!(formatted, s);
        }

        test_impl("1970-01-01T00:00:00Z", 0);
        test_impl("1970-01-01T00:00:00.000Z", 3);
        test_impl("1970-01-01T00:00:00.000000000Z", 9);
        test_impl("1970-01-02T00:00:00Z", 0);
        test_impl("1970-03-01T00:00:00Z", 0);
        test_impl("1974-01-01T00:00:00Z", 0);
        test_impl("2018-01-01T00:00:00Z", 0);
        test_impl("2018-09-02T05:49:10.123456789Z", 9);
        test_impl("0001-01-01T00:00:00.000000000Z", 9);
        test_impl("9999-12-31T23:59:59.999999999Z", 9);
    }

    #[test]
    fn test_parse_alt() {
        fn test_impl(alt: &str, parse: &str) {
            let reference = TmUtc::parse_rfc_3339(alt).unwrap();
            let parsed = TmUtc::parse_rfc_3339(parse).unwrap();
            assert_eq!(reference, parsed, "{} - {}", alt, parse);
        }

        // alternative spelling
        test_impl("1970-01-01 00:00:00Z", "1970-01-01T00:00:00Z");
        test_impl("1970-01-01 00:00:00Z", "1970-01-01t00:00:00Z");
        test_impl("1970-01-01 00:00:00Z", "1970-01-01 00:00:00z");
        // leap second is rounded down
        test_impl("2016-12-31 23:59:59Z", "2016-12-31 23:59:60Z");
        // TZ offset
        test_impl("1970-01-01 00:00:00Z", "1970-01-01T03:00:00+03:00");
        test_impl("1970-01-01 00:00:00Z", "1969-12-31 22:15:00-01:45");
    }

    #[test]
    fn test_parse_incorrect_inputs() {
        fn test_impl(s: &str) {
            assert!(TmUtc::parse_rfc_3339(s).is_err(), "{}", s);
        }

        test_impl("1970-01-01T00:00:61Z");
        test_impl("1970-01-01T00:60:61Z");
        test_impl("1970-01-01T24:00:61Z");
        test_impl("1970-01-01T00:00:00.Z");
        test_impl("1970-01-32T00:00:00Z");
        test_impl("1970-02-29T00:00:00Z");
        test_impl("1980-02-30T00:00:00Z");
        test_impl("1980-13-01T00:00:00Z");
        test_impl("1970-01-01T00:00:00");
        test_impl("1970-01-01T00:00Z");
    }

    #[test]
    fn test_fmt_max_duration() {
        // Simply check that there are no integer overflows.
        // I didn't check that resulting strings are correct.
        assert_eq!(
            "-292277022657-01-27T08:29:52.000000000Z",
            format!("{}", TmUtc::from_protobuf_timestamp(i64::MIN, 0))
        );
        assert_eq!(
            "+292277026596-12-04T15:30:07.999999999Z",
            format!("{}", TmUtc::from_protobuf_timestamp(i64::MAX, 999_999_999))
        );
    }
}
