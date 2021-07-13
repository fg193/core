use chrono::{DateTime, FixedOffset};

pub type Time = DateTime<FixedOffset>;

impl super::DataType<'_> for Time {
    fn type_name() -> &'static str {
        "time"
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    #[test]
    fn test_serialize() -> Result<(), Box<dyn Error>> {
        let time: crate::data_types::Time =
            chrono::DateTime::parse_from_rfc2822(
                "Mon, 02 Jan 2006 15:04:05 -0700",
            )?;

        let ret = serde_json::to_string(&time)?;
        assert_eq!(ret, r#""2006-01-02T15:04:05-07:00""#);
        Ok(())
    }
}
