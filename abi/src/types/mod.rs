mod request;
mod reservation;
mod reservation_filter;
mod reservation_query;
mod reservation_status;

use std::ops::Bound;

use crate::{convert_to_utc_time, Error};
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use sqlx::postgres::types::PgRange;

pub fn validate_range(start: Option<&Timestamp>, end: Option<&Timestamp>) -> Result<(), Error> {
    if start.is_none() || end.is_none() {
        return Err(crate::Error::InvalidTime);
    }

    let start = start.unwrap();
    let end = end.unwrap();

    if start.seconds >= end.seconds {
        return Err(crate::Error::InvalidTime);
    }
    Ok(())
}

pub fn get_timespan(start: Option<&Timestamp>, end: Option<&Timestamp>) -> PgRange<DateTime<Utc>> {
    let start = convert_to_utc_time(start.as_ref().unwrap());
    let end = convert_to_utc_time(end.as_ref().unwrap());

    PgRange {
        start: Bound::Included(start),
        end: Bound::Included(end),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_range_should_allow_correct_range() {
        let start = Timestamp {
            seconds: 1,
            nanos: 0,
        };
        let end = Timestamp {
            seconds: 2,
            nanos: 0,
        };

        assert!(validate_range(Some(&start), Some(&end)).is_ok());
    }

    #[test]
    fn validate_range_should_reject_invalid_range() {
        let start = Timestamp {
            seconds: 2,
            nanos: 0,
        };
        let end = Timestamp {
            seconds: 1,
            nanos: 0,
        };

        assert!(validate_range(Some(&start), Some(&end)).is_err());
    }

    #[test]
    fn get_timstamp_should_work_for_valid_start_end() {
        let start = Timestamp {
            seconds: 1,
            nanos: 0,
        };
        let end = Timestamp {
            seconds: 2,
            nanos: 0,
        };

        let range = get_timespan(Some(&start), Some(&end));

        assert_eq!(range.start, Bound::Included(convert_to_utc_time(&start)));
        assert_eq!(range.end, Bound::Included(convert_to_utc_time(&end)));
    }
}
