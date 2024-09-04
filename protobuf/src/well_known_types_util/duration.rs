use crate::well_known_types::duration::Duration;
use crate::SpecialFields;

impl Duration {
    /// Zero seconds zero nanoseconds.
    pub const ZERO: Duration = Duration {
        seconds: 0,
        nanos: 0,
        special_fields: SpecialFields::new(),
    };
}

/// Convert from `std::time::Duration`.
///
/// # Panics
///
/// If `std::time::Duration` value is outside of `Duration` supported range.
impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Self {
        Duration {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
            ..Default::default()
        }
    }
}

/// Convert to `std::time::Duration`.
///
/// This conversion might be lossy if `std::time::Duration` precision is smaller than nanoseconds.
///
/// # Panics
///
/// If `Duration` value is outside of `std::time::Duration` supported range.
impl From<Duration> for std::time::Duration {
    fn from(val: Duration) -> Self {
        assert!(val.seconds >= 0);
        std::time::Duration::from_secs(val.seconds as u64)
            + std::time::Duration::from_nanos(val.nanos as u64)
    }
}

#[cfg(test)]
mod test {
    use crate::well_known_types::duration::Duration;

    #[test]
    fn to_from_duration() {
        fn to_from(duration: Duration, std_time_duration: std::time::Duration) {
            assert_eq!(duration, Duration::from(std_time_duration));
            assert_eq!(
                std_time_duration,
                Into::<std::time::Duration>::into(duration)
            );
        }

        to_from(Duration::ZERO, std::time::Duration::from_secs(0));
        to_from(
            Duration {
                seconds: 4,
                nanos: 123_000_000,
                ..Default::default()
            },
            std::time::Duration::from_millis(4_123),
        );
    }
}
