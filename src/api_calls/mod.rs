//! [documentation](https://docs.modrinth.com/api-spec)

pub mod project;
pub mod tag;
pub mod team;
pub mod user;
pub mod version;
pub mod version_file;

use crate::{
    request::RequestBuilderCustomSend,
    structures,
    url_ext::{UrlJoinAll, UrlWithQuery},
    Error, Ferinth, Result, API_BASE_URL,
};

/// Verify that the `inputs` are Modrinth ID or slug compliant
pub fn check_id_slug<S: AsRef<str>>(inputs: &[S]) -> Result<()> {
    for input in inputs {
        // Regex from the [Modrinth documentation](https://docs.modrinth.com/api-spec/#tag/project_model)
        if !lazy_regex::regex_is_match!(r#"^[\w!@$()`.+,"\-']{3,64}$"#, input.as_ref()) {
            return Err(Error::InvalidIDorSlug);
        }
    }
    Ok(())
}

/// Verify that the given `inputs` are SHA1 compliant
pub(crate) fn check_sha1_hash<S: AsRef<str>>(inputs: &[S]) -> Result<()> {
    for input in inputs {
        if !lazy_regex::regex_is_match!("^[a-f0-9]{40}$", input.as_ref()) {
            return Err(Error::InvalidSHA1);
        }
    }
    Ok(())
}
