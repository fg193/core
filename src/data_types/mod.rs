mod i18n;
mod link;
mod text;
mod time;

pub use i18n::I18n;
pub use link::Link;
pub use text::Text;
pub use time::Time;

pub trait DataType<'de>: serde::Serialize + serde::Deserialize<'de> {
    fn type_name() -> &'static str;
}
