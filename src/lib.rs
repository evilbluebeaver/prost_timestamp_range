use std::ops::Bound;

use chrono::{DateTime, Utc};

#[derive(Clone, Copy, PartialEq, Eq, Hash, prost::Message)]
pub struct ProstTimestampRange {
    #[prost(int64, optional, tag = "1")]
    pub start: Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub end: Option<i64>,
    #[prost(bool, optional, tag = "3")]
    pub start_is_excluded: Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub end_is_included: Option<bool>,
}

impl From<ProstTimestampRange> for (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>) {
    fn from(range: ProstTimestampRange) -> Self {
        let start = range
            .start
            .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0));
        let end = range
            .end
            .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0));

        let start_bound = match start {
            Some(start) if range.start_is_excluded.unwrap_or_default() => Bound::Excluded(start),
            Some(start) => Bound::Included(start),
            None => Bound::Unbounded,
        };
        let end_bound = match end {
            Some(end) if range.end_is_included.unwrap_or_default() => Bound::Included(end),
            Some(end) => Bound::Excluded(end),
            None => Bound::Unbounded,
        };

        (start_bound, end_bound)
    }
}

impl From<(Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)> for ProstTimestampRange {
    fn from(range: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)) -> Self {
        let (start, start_is_excluded) = match range.0 {
            Bound::Included(start) => (Some(start), None),
            Bound::Excluded(start) => (Some(start), Some(true)),
            Bound::Unbounded => (None, None),
        };
        let (end, end_is_included) = match range.1 {
            Bound::Included(end) => (Some(end), Some(true)),
            Bound::Excluded(end) => (Some(end), None),
            Bound::Unbounded => (None, None),
        };
        ProstTimestampRange {
            start: start.map(|start| start.timestamp()),
            end: end.map(|end| end.timestamp()),
            start_is_excluded,
            end_is_included,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_all_bound_combinations() {
        let start = DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        let end = DateTime::from_timestamp(1_700_003_600, 0).unwrap();

        for (start_bound, end_bound, start_is_excluded, end_is_included) in [
            (Bound::Included(start), Bound::Excluded(end), None, None),
            (
                Bound::Included(start),
                Bound::Included(end),
                None,
                Some(true),
            ),
            (
                Bound::Excluded(start),
                Bound::Excluded(end),
                Some(true),
                None,
            ),
            (
                Bound::Excluded(start),
                Bound::Included(end),
                Some(true),
                Some(true),
            ),
            (Bound::Unbounded, Bound::Excluded(end), None, None),
            (Bound::Unbounded, Bound::Included(end), None, Some(true)),
            (Bound::Included(start), Bound::Unbounded, None, None),
            (Bound::Excluded(start), Bound::Unbounded, Some(true), None),
            (Bound::Unbounded, Bound::Unbounded, None, None),
            (Bound::Included(end), Bound::Excluded(start), None, None),
        ] {
            let encoded: ProstTimestampRange = (start_bound, end_bound).into();

            assert_eq!(encoded.start_is_excluded, start_is_excluded);
            assert_eq!(encoded.end_is_included, end_is_included);
            assert_eq!(
                <(Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)>::from(encoded),
                (start_bound, end_bound)
            );
        }
    }

    #[test]
    fn defaults_to_the_legacy_half_open_range() {
        let range = ProstTimestampRange {
            start: Some(1_700_000_000),
            end: Some(1_700_003_600),
            start_is_excluded: None,
            end_is_included: None,
        };

        let (start, end): (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>) = range.into();

        assert!(matches!(start, Bound::Included(_)));
        assert!(matches!(end, Bound::Excluded(_)));
    }

    #[test]
    fn treats_invalid_timestamps_as_unbounded() {
        let range = ProstTimestampRange {
            start: Some(i64::MAX),
            end: Some(1_700_003_600),
            start_is_excluded: Some(true),
            end_is_included: None,
        };

        let (start, end): (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>) = range.into();

        assert!(matches!(start, Bound::Unbounded));
        assert!(matches!(end, Bound::Excluded(_)));
    }
}
