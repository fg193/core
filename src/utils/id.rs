use data_encoding::Encoding;
use data_encoding_macro::new_encoding;
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    serialize::ToSql,
    sql_types::{BigInt, HasSqlType},
};
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryInto;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

const CROCKFORD: Encoding = new_encoding! {
    symbols: "0123456789ABCDEFGHJKMNPQRSTVWXYZ",
    ignore: " '(),-./:;_",
    translate_from: "oilz",
    translate_to: "0112",
    bit_order: MostSignificantFirst,
    padding: None,
    check_trailing_bits: false,
};

#[derive(
    AsExpression,
    Clone,
    Copy,
    Debug,
    Eq,
    FromSqlRow,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[sql_type = "BigInt"]
pub struct Id(i64);

impl Id {
    pub fn new() -> Id {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        let mut rng = rand::thread_rng();
        let serial: u64 = rng.gen_range(1..64);
        Id::from(timestamp << 6 | serial)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        Ok(Id::from(String::deserialize(deserializer)?.as_str()))
    }
}

impl<Db> FromSql<BigInt, Db> for Id
where
    Db: Backend + HasSqlType<BigInt>,
    i64: FromSql<BigInt, Db>,
{
    fn from_sql(
        bytes: Option<&Db::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        let i: i64 = <i64 as FromSql<BigInt, Db>>::from_sql(bytes)?;
        Ok(Id(i))
    }
}

impl From<i64> for Id {
    fn from(i: i64) -> Self {
        Id(i)
    }
}

impl From<u64> for Id {
    fn from(u: u64) -> Self {
        Id(u as i64)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Id(
            match CROCKFORD.decode((s.to_ascii_uppercase() + "0").as_bytes()) {
                Ok(v) => i64::from_be_bytes(v.try_into().unwrap()) >> 4,
                Err(e) => {
                    dbg!(e);
                    0
                }
            },
        )
    }
}

impl From<actix_identity::Identity> for Id {
    fn from(identity: actix_identity::Identity) -> Self {
        match identity.identity() {
            Some(s) => Id::from(s.as_str()),
            None => Id::from(0u64),
        }
    }
}

impl Serialize for Id {
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&String::from(self))
    }
}

impl<Db> ToSql<BigInt, Db> for Id
where
    Db: Backend + HasSqlType<BigInt>,
{
    fn to_sql<W: Write>(
        &self,
        out: &mut diesel::serialize::Output<W, Db>,
    ) -> diesel::serialize::Result {
        <i64 as ToSql<BigInt, Db>>::to_sql(&self.0, out)
    }
}

impl From<&Id> for i64 {
    fn from(id: &Id) -> Self {
        id.0
    }
}

impl From<&Id> for u64 {
    fn from(id: &Id) -> Self {
        id.0 as u64
    }
}

impl From<&Id> for String {
    fn from(id: &Id) -> Self {
        let slice = &(id.0 << 4).to_be_bytes();
        let encoded = CROCKFORD.encode(slice).to_ascii_uppercase();
        [&encoded[..4], &encoded[4..8], &encoded[8..12]].join("-")
    }
}

#[cfg(test)]
mod test {
    use crate::utils::id::Id;
    use std::collections::HashSet;

    #[test]
    fn test_from() {
        assert_eq!(
            Id::from("80FF-0F0F-55AA"),
            Id::from(0x_0401_EF03_C0F2_954A_u64)
        );
    }

    #[test]
    fn test_palindrome() {
        let mut generated = HashSet::new();

        for _ in 0..100000 {
            let i = Id::new();
            assert_eq!(generated.get(&i64::from(&i)), None);
            generated.insert(i64::from(&i));

            let s = String::from(&i);
            assert_eq!(i, Id::from(s.as_str()));
        }
    }
}
