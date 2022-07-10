pub mod project_structs;
pub mod tag_structs;
pub mod user_structs;
pub mod version_structs;

pub type UtcTime = chrono::DateTime<chrono::Utc>;
pub type Number = usize;
pub type URL = url::Url;
/// A base 62 number stored as a string
pub type ID = String;

use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use std::borrow::Cow;

fn deserialise_optional_url<'de, D: Deserializer<'de>>(de: D) -> Result<Option<URL>, D::Error> {
    let intermediate = <Option<Cow<'de, str>>>::deserialize(de)?;
    match intermediate.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => URL::parse(s).map_or_else(
            |err| {
                Err(Error::invalid_value(
                    Unexpected::Str(s),
                    &err.to_string().as_str(),
                ))
            },
            |ok| Ok(Some(ok)),
        ),
    }
}
