pub mod project;
pub mod tag;
pub mod team;
pub mod user;
pub mod version;
pub mod version_file;

use crate::{Error, Result};

/// Verify that a given string `input` is compliant with Modrinth IDs or slugs
pub(crate) fn check_id_slug(input: &str) -> Result<()> {
    // regex taken from [Modrinth documentation](https://docs.modrinth.com/api-spec/#tag/project_model)
    lazy_regex::regex_is_match!(r#"^[\w!@$()`.+,"\-']{3,64}$"#, input)
        .then_some(())
        .ok_or(Error::NotBase62)
}

/// Verify that a given string `input` is SHA1 compliant
pub(crate) fn check_sha1_hash(input: &str) -> Result<()> {
    // Check that all 40 characters are SHA1 compliant
    match lazy_regex::regex_is_match!("^[a-f0-9]{40}$", input) {
        true => Ok(()),
        false => Err(Error::NotSHA1),
    }
}
