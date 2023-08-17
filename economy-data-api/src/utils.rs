use std::{fmt, str::FromStr};

use serde::{de, Deserialize, Deserializer};

/// Serde deserialization decorator to map empty Strings to None,
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

/// Mongo string sanitization
pub fn sanitize_mongo_string(s: String) -> String {
    let cleaned = s.replace(should_replace_mongo_char, "");

    cleaned
}

fn should_replace_mongo_char(c: char) -> bool {
    if c.is_alphanumeric()
        || c == '+'
        || c == '-'
        || c == '%'
        || c == '\''
        || c == ' '
        || c == ','
        || c == '.'
    {
        return false;
    }

    true
}
