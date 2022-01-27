use crate::{Error, Result};

pub mod mod_calls;
pub mod tag_calls;
pub mod user_calls;
pub mod version_calls;

/// Verify that a given string `input` is base62 or not
pub(crate) fn check_id_slug(input: &str) -> Result<()> {
    match regex::Regex::new("[^a-zA-Z0-9-]")
        .unwrap()
        .is_match(input)
    {
        true => Err(Error::NotBase62),
        false => Ok(()),
    }
}

/// Verify that a given string `input` is a SHA1 hash
pub(crate) fn check_sha1_hash(input: &str) -> Result<()> {
    match regex::Regex::new("^[a-f0-9]{40}$")
        .unwrap()
        .is_match(input)
    {
        true => Ok(()),
        false => Err(Error::NotValidHash),
    }
}