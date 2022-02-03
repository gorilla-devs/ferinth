pub mod project_structs;
pub mod tag_structs;
pub mod user_structs;
pub mod version_structs;

pub type Datetime = chrono::DateTime<chrono::Utc>;
/// A base 62 number stored as a string
pub type ID = String;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack,
}
