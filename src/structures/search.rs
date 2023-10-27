use super::{
    project::{ProjectSupportRange, ProjectType},
    *,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sort {
    Relevance,
    /// Sorts matches by the number of downloads
    Downloads,
    /// Sorts matches by the number of followers
    Follows,
    /// Sorts by the time of initial creation
    Newest,
    /// Sorts by the time of the latest update
    Updated,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Facet {
    /// Mod loader or category to filter
    Categories(String),
    Versions(String),
    /// License ID to filter
    License(String),
    ProjectType(project::ProjectType),
}

impl Serialize for Facet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let output = match self {
            Facet::Categories(category) => format!("categories:{}", category),
            Facet::Versions(version) => format!("versions:{}", version),
            Facet::License(license_id) => format!("license:{}", license_id),
            Facet::ProjectType(project_type) => {
                format!(
                    "project_type:{}",
                    serde_json::to_value(project_type)
                        .unwrap()
                        .as_str()
                        .unwrap()
                )
            }
        };
        serializer.collect_str(&output)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Response {
    pub hits: Vec<SearchHit>,
    /// The number of results that were skipped by the query
    pub offset: Number,
    /// The number of results that were returned by the query
    pub limit: Number,
    /// The total number of results that match the query
    pub total_hits: Number,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchHit {
    /// The project's slug, used for vanity URLs.
    /// This can change at any time, so use the [`Project::id`] for long term storage.
    pub slug: String,
    pub title: String,
    /// A short description of the project
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: ProjectSupportRange,
    pub server_side: ProjectSupportRange,
    pub project_type: ProjectType,
    pub downloads: Number,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub icon_url: Option<Url>,
    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<Number>,
    pub project_id: ID,
    #[serde(rename = "versions")]
    /// A list of all of the game versions supported by the project
    pub game_versions: Vec<String>,
    pub follows: Number,
    pub date_created: UtcTime,
    pub date_modified: UtcTime,
    /// The latest version of Minecraft that this project supports
    pub latest_version: String,
    /// The SPDX license ID of a project
    pub license: String,
    pub gallery: Vec<Url>,
    pub featured_gallery: Option<Url>,
}
