pub mod mod_structs;
pub mod user_structs;
pub mod version_structs;

pub type Datetime = chrono::DateTime<chrono::Utc>;
/// A base 62 number encoded as a string
pub type ID = String;
