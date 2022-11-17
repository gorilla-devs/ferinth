use super::*;
use crate::structures::project::{ProjectSupportRange, ProjectType};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchResult {
    /// The project's slug.
    /// This can change at any time, so use the `project_id` for long term storage
    pub slug: String,
    /// The project's title or display name
    pub title: String,
    /// A short description of the project
    pub description: String,
    /// A list of categories the project is in
    pub categories: Vec<String>,
    /// The project's client side support range
    pub client_side: ProjectSupportRange,
    /// The project's server side support range
    pub server_side: ProjectSupportRange,
    /// The project type of the project
    pub project_type: ProjectType,
    /// The total number of downloads the project has
    pub downloads: Number,
    #[serde(deserialize_with = "deserialise_optional_url")]
    /// The link to the project's icon
    pub icon_url: Option<Url>,
    /// The project's ID
    pub project_id: ID,
    /// The username of the project's author
    pub author: String,
    /// A list of categories the project is in which are not secondary
    pub display_categories: Vec<String>,
    /// A list of the minecraft versions supported by the project
    pub versions: Vec<String>,
    /// The total number of users following the project
    pub follows: Number,
    /// The date and time the project was created
    pub date_created: UtcTime,
    /// The date and time the project was modified
    pub date_modified: UtcTime,
    /// The latest version of minecraft that this project supports
    pub latest_version: Option<String>,
    /// The project's license
    pub license: String,
    /// All gallery images attached to the project
    pub gallery: Vec<Url>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SortingMethod {
    Relevance,
    Downloads,
    Follows,
    Newest,
    Updated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    /// The list of results
    pub hits: Vec<SearchResult>,
    /// The number of results that were skipped by the query
    pub offset: Number,
    /// The number of results that were returned by the query
    pub limit: Number,
    /// The total number of results that match the query
    pub total_hits: Number,
}
