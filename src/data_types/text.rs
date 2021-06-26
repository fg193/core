pub type Text = String;

impl super::DataType<'_> for Text {
    fn type_name() -> &'static str {
        "text"
    }
}
