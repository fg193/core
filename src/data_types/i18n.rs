#[derive(serde::Serialize, serde::Deserialize)]
pub struct I18n {
    #[serde(skip)]
    pub id: i64,

    #[serde(flatten)]
    pub value: std::collections::HashMap<String, String>,
}

impl super::DataType<'_> for I18n {
    fn type_name() -> &'static str {
        "i18n"
    }
}

#[cfg(test)]
mod test {
    use simple_error::bail;
    use std::array::IntoIter;
    use std::collections::HashMap;
    use std::error::Error;
    use std::iter::FromIterator;

    #[test]
    fn test_serialize() -> Result<(), Box<dyn Error>> {
        let i18n = crate::data_types::I18n {
            id: 123,
            value: HashMap::<String, String>::from_iter(IntoIter::new([
                ("zh-CN".into(), "中文".into()),
                ("en-US".into(), "English".into()),
            ])),
        };

        let ret = serde_json::to_string(&i18n)?;
        match ret.as_str() {
            r#"{"zh-CN":"中文","en-US":"English"}"# => Ok(()),
            r#"{"en-US":"English","zh-CN":"中文"}"# => Ok(()),
            _ => Err(bail!("unexpected output: {}", ret)),
        }
    }
}
