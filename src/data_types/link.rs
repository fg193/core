#[derive(serde::Serialize, serde::Deserialize)]
pub struct Link {
    pub id: u64,
    pub label: super::I18n,
}

impl super::DataType<'_> for Link {
    fn type_name() -> &'static str {
        "link"
    }
}

#[cfg(test)]
mod test {
    use std::array::IntoIter;
    use std::collections::HashMap;
    use std::error::Error;
    use std::iter::FromIterator;

    #[test]
    fn test_serialize() -> Result<(), Box<dyn Error>> {
        let link = crate::data_types::Link {
            id: 123,
            label: crate::data_types::I18n {
                id: 456,
                value: HashMap::<String, String>::from_iter(IntoIter::new([
                    ("zh-CN".into(), "中文".into()),
                    ("en-US".into(), "English".into()),
                ])),
            },
        };

        let ret = serde_json::to_string(&link)?;
        match ret.as_str() {
            r#"{"id":123,"label":{"zh-CN":"中文","en-US":"English"}}"# => {
                Ok(())
            }
            r#"{"id":123,"label":{"en-US":"English","zh-CN":"中文"}}"# => {
                Ok(())
            }
            _ => Err(ret.into()),
        }
    }
}
