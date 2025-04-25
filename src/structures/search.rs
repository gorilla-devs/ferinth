use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    ProjectType(project::ProjectType),
    /// Mod loader or category to filter
    Categories(String),
    /// Game versions to filter
    Versions(String),
    OpenSource(bool),
    /// License ID to filter
    License(String),
    /// A custom facet
    ///
    /// [documentation](https://docs.modrinth.com/api-spec#tag/projects/operation/searchProjects)
    Custom {
        /// The type of metadata to filter
        _type: String,
        /// The comparison to use
        ///
        /// Can be `=`/`:`, `!=`, `>`, `>=`, `<`, `<=`
        operation: String,
        /// The value to compare against
        value: String,
    },
}

impl Serialize for Facet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let output = match self {
            Facet::ProjectType(project_type) => {
                format!("project_type:{project_type:?}",)
            }
            Facet::Categories(category) => format!("categories: {category}"),
            Facet::Versions(version) => format!("versions: {version}"),
            Facet::OpenSource(bool) => format!("open_source: {bool}"),
            Facet::License(license_id) => format!("license: {license_id}"),
            Facet::Custom {
                _type,
                operation,
                value,
            } => format!("{_type} {operation} {value}"),
        };
        serializer.collect_str(&output)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Response {
    pub hits: Vec<SearchHit>,
    /// The number of results that were skipped by the query
    pub offset: Int,
    /// The number of results that were returned by the query
    pub limit: Int,
    /// The total number of results that match the query
    pub total_hits: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchHit {
    /// The project's slug, used for vanity URLs.
    /// This can change at any time, so use the [`Self::project_id`] for long term storage.
    pub slug: Option<String>,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: project::ProjectSupportRange,
    pub server_side: project::ProjectSupportRange,
    pub project_type: project::ProjectType,
    pub downloads: Int,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub icon_url: Option<Url>,
    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<Int>,
    /// The ID of the moderation thread associated with this project
    pub thread_id: Option<ID>,
    pub monetization_status: Option<project::MonetizationStatus>,
    pub project_id: ID,
    /// Username of the project's authour
    pub author: String,
    /// A list of the project's primary/featured categories
    pub display_categories: Vec<String>,
    #[serde(rename = "versions")]
    /// A list of all of the game versions supported by the project
    pub game_versions: Vec<String>,
    pub follows: Int,
    pub date_created: UtcTime,
    pub date_modified: UtcTime,
    /// The latest game version that this project supports
    pub latest_version: String,
    /// The SPDX license ID of a project
    pub license: String,
    pub gallery: Vec<Url>,
    pub featured_gallery: Option<Url>,
}
