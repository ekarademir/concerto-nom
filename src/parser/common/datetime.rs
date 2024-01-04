use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::recognize,
    error::context,
    multi::count,
    sequence::{pair, tuple},
    Parser,
};

use crate::parser::CResult;

fn year<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("Year", recognize(count(one_of("1234567890"), 4)))(input)
}

fn month<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "Month",
        recognize(alt((
            pair(char('0'), one_of("1234567890")),
            pair(char('1'), one_of("1234567890")),
        ))),
    )(input)
}

fn day<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "Day",
        recognize(alt((
            pair(one_of("012"), one_of("1234567890")),
            pair(char('3'), one_of("01")),
        ))),
    )(input)
}

fn hour<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "Hour",
        recognize(alt((
            pair(one_of("01"), one_of("1234567890")),
            pair(char('2'), one_of("0123")),
        ))),
    )(input)
}

fn minute<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "Minute",
        recognize(pair(one_of("012345"), one_of("1234567890"))),
    )(input)
}

fn second<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "Second",
        recognize(pair(one_of("012345"), one_of("1234567890"))),
    )(input)
}

fn year_month_day<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "YearMonthDay",
        recognize(tuple((year, char('-'), month, char('-'), day))),
    )(input)
}

fn hour_minute_second<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "HourMinuteSecond",
        recognize(tuple((hour, char(':'), minute, char(':'), second))),
    )(input)
}

/// As described in the spec https://concerto.accordproject.org/docs/design/specification/model-properties/
pub(crate) fn datetime_value<'a>(input: &'a str) -> CResult<&'a str, String> {
    let ymd = context("YYYY-MM-DD", year_month_day);
    let ymd_hms = context(
        "YYYY-MM-DDTHH:mm:ssZ",
        recognize(tuple((
            year_month_day,
            char('T'),
            hour_minute_second,
            char('Z'),
        ))),
    );
    let ymd_hms_hm = context(
        "YYYY-MM-DDTHH:mm:ss±HH:mm",
        recognize(tuple((
            year_month_day,
            char('T'),
            hour_minute_second,
            one_of("+-"),
            hour,
            char(':'),
            minute,
        ))),
    );
    let ymd_hms_s = context(
        "YYYY-MM-DDTHH:mm:ss.SZ",
        recognize(tuple((
            year_month_day,
            char('T'),
            hour_minute_second,
            char('.'),
            one_of("1234567890"),
            char('Z'),
        ))),
    );
    let ymd_hms_ss = context(
        "YYYY-MM-DDTHH:mm:ss.SSZ",
        recognize(tuple((
            year_month_day,
            char('T'),
            hour_minute_second,
            char('.'),
            one_of("1234567890"),
            one_of("1234567890"),
            char('Z'),
        ))),
    );
    let ymd_hms_sss = context(
        "YYYY-MM-DDTHH:mm:ss.SSSZ",
        recognize(tuple((
            year_month_day,
            char('T'),
            hour_minute_second,
            char('.'),
            one_of("1234567890"),
            one_of("1234567890"),
            one_of("1234567890"),
            char('Z'),
        ))),
    );
    let ymd_hms_hm_s = context(
        "YYYY-MM-DDTHH:mm:ss.S±HH:mm",
        recognize(tuple((
            year_month_day,
            char('T'),
            hour_minute_second,
            char('.'),
            one_of("1234567890"),
            one_of("+-"),
            hour,
            char(':'),
            minute,
        ))),
    );
    let ymd_hms_hm_ss = context(
        "YYYY-MM-DDTHH:mm:ss.SS±HH:mm",
        recognize(tuple((
            year_month_day,
            char('T'),
            hour_minute_second,
            char('.'),
            one_of("1234567890"),
            one_of("1234567890"),
            one_of("+-"),
            hour,
            char(':'),
            minute,
        ))),
    );
    let ymd_hms_hm_sss = context(
        "YYYY-MM-DDTHH:mm:ss.SSS±HH:mm",
        recognize(tuple((
            year_month_day,
            char('T'),
            hour_minute_second,
            char('.'),
            one_of("1234567890"),
            one_of("1234567890"),
            one_of("1234567890"),
            one_of("+-"),
            hour,
            char(':'),
            minute,
        ))),
    );

    context(
        "DateTime",
        alt((
            ymd_hms_hm_sss,
            ymd_hms_hm_ss,
            ymd_hms_hm_s,
            ymd_hms_sss,
            ymd_hms_ss,
            ymd_hms_s,
            ymd_hms_hm,
            ymd_hms,
            ymd,
        ))
        .map(|s: &'a str| s.to_string()),
    )(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_datetime_value() {
        assert_eq!(
            super::datetime_value("2024-01-04"),
            Ok(("", "2024-01-04".to_string())),
            "Parses YYYY-MM-DD"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T00:12:42Z"),
            Ok(("", "2024-01-04T00:12:42Z".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ssZ"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T00:12:42-01:00"),
            Ok(("", "2024-01-04T00:12:42-01:00".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss-HH:mm"
        );
        assert_eq!(
            super::datetime_value("2024-01-04T00:12:42+04:30"),
            Ok(("", "2024-01-04T00:12:42+04:30".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss+HH:mm"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T12:13:14.1Z"),
            Ok(("", "2024-01-04T12:13:14.1Z".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.SZ"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T12:13:14.12Z"),
            Ok(("", "2024-01-04T12:13:14.12Z".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.SSZ"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T12:13:14.123Z"),
            Ok(("", "2024-01-04T12:13:14.123Z".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.SSSZ"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T01:02:03.4+04:00"),
            Ok(("", "2024-01-04T01:02:03.4+04:00".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.S+HH:mm"
        );
        assert_eq!(
            super::datetime_value("2024-01-04T01:02:03.4-05:15"),
            Ok(("", "2024-01-04T01:02:03.4-05:15".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.S-HH:mm"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T01:02:03.45+04:00"),
            Ok(("", "2024-01-04T01:02:03.45+04:00".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.SS+HH:mm"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T01:02:03.45-05:15"),
            Ok(("", "2024-01-04T01:02:03.45-05:15".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.SS-HH:mm"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T01:02:03.456+04:00"),
            Ok(("", "2024-01-04T01:02:03.456+04:00".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.SSS+HH:mm"
        );

        assert_eq!(
            super::datetime_value("2024-01-04T01:02:03.456-05:15"),
            Ok(("", "2024-01-04T01:02:03.456-05:15".to_string())),
            "Parses YYYY-MM-DDTHH:mm:ss.SSS-HH:mm"
        );
    }
}
