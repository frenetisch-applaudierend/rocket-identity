use std::collections::HashMap;

use rocket::{
    serde::{de::Visitor, json::Value, Deserialize, Serialize},
    time::{error, OffsetDateTime},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub nbf: NumericDate,
    pub iat: NumericDate,
    pub exp: NumericDate,

    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Entitlement {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct NumericDate(OffsetDateTime);

impl Serialize for NumericDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let timestamp = self.0.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }
}

impl<'de> Deserialize<'de> for NumericDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: rocket::serde::Deserializer<'de>,
    {
        deserializer.deserialize_i64(NumericDateVisitor)
    }
}

impl TryFrom<i64> for NumericDate {
    type Error = error::ComponentRange;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(NumericDate(OffsetDateTime::from_unix_timestamp(value)?))
    }
}

impl From<OffsetDateTime> for NumericDate {
    fn from(value: OffsetDateTime) -> Self {
        Self(value)
    }
}

struct NumericDateVisitor;

impl<'a> Visitor<'a> for NumericDateVisitor {
    type Value = NumericDate;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^63 and 2^63")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: rocket::serde::de::Error,
    {
        self.numeric_date(v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: rocket::serde::de::Error,
    {
        self.numeric_date(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: rocket::serde::de::Error,
    {
        self.numeric_date(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: rocket::serde::de::Error,
    {
        self.numeric_date(v)
    }
}

impl NumericDateVisitor {
    fn numeric_date<E>(&self, v: impl Into<i64>) -> Result<NumericDate, E>
    where
        E: rocket::serde::de::Error,
    {
        NumericDate::try_from(v.into()).map_err(E::custom)
    }
}
