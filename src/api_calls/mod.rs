pub mod project;
pub mod tag;
pub mod team;
pub mod user;
pub mod version;
pub mod version_file;

use crate::{Error, Result};

/// Verify that a given string `input` is base62 and Modrinth slugs compliant
pub(crate) fn check_id_slug(input: &str) -> Result<()> {
    // Check if there is any character that isn't valid in base62 e.g. '/'
    match lazy_regex::regex_is_match!("[^a-zA-Z0-9-_]", input) {
        true => Err(Error::NotBase62),
        false => Ok(()),
    }
}

/// Verify that a given string `input` is SHA1 compliant
pub(crate) fn check_sha1_hash(input: &str) -> Result<()> {
    // Check that all 40 characters are SHA1 compliant
    match lazy_regex::regex_is_match!("^[a-f0-9]{40}$", input) {
        true => Ok(()),
        false => Err(Error::NotSHA1),
    }
}
