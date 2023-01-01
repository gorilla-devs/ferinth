use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sort {
    Relevance,
    /// Sorts matches by the number of downloads
    Downloads,
    /// Sorts matches by the number of followers
    Follows,
    /// Sorst by the time of initial creation
    Newest,
    /// Sorts by the time of the latest update
    Updated,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Relevance => "relevance",
                Self::Downloads => "downloads",
                Self::Follows => "follows",
                Self::Newest => "newest",
                Self::Updated => "updated",
            }
        )
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
                    serde_json::to_string(project_type).unwrap()
                )
            }
        };
        serializer.collect_str(&output)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Response {
    pub hits: Vec<project::Project>,
    pub offset: Number,
    pub limit: Number,
    pub total_hits: Number,
}
