//! Models related to projects
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/project_model)

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    /// The project's slug, used for vanity URLs.
    /// This can change at any time, so use the [`Self::id`] for long term storage.
    pub slug: String,
    pub title: String,
    /// A short description of the project
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: ProjectSupportRange,
    pub server_side: ProjectSupportRange,
    /// A long form description of the project
    pub body: String,
    pub status: ProjectStatus,
    /// The status requested. Only visible to those with appropriate permissions
    pub requested_status: Option<RequestedStatus>,
    /// A list of categories which are searchable but non-primary
    pub additional_categories: Vec<String>,
    /// A link to submit bugs or issues with the project
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub issues_url: Option<Url>,
    /// A link to the project's source code
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub source_url: Option<Url>,
    /// A link to the project's wiki page or other relevant information
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub wiki_url: Option<Url>,
    /// The project's Discord server invite
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub discord_url: Option<Url>,
    pub donation_urls: Vec<DonationLink>,
    pub project_type: ProjectType,
    pub downloads: Int,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub icon_url: Option<Url>,
    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<Int>,
    /// The ID of the moderation thread associated with this project
    pub thread_id: ID,
    pub monetization_status: MonetizationStatus,
    pub id: ID,
    /// The ID of the team that has ownership of this project
    pub team: ID,
    pub published: UtcTime,
    pub updated: UtcTime,
    /// The date the project's status was approved
    pub approved: Option<UtcTime>,
    pub queued: Option<UtcTime>,
    pub followers: Int,
    pub license: License,
    /// A list of the version IDs of the project.
    /// This will only ever be empty if the project is a draft.
    pub versions: Vec<ID>,
    /// A list of all of the game versions supported by the project
    pub game_versions: Vec<String>,
    /// A list of all of the loaders supported by the project
    pub loaders: Vec<String>,
    /// A list of images that have been uploaded to the project's gallery
    pub gallery: Vec<GalleryItem>,
    // The ID of the organisation that owns this project
    pub organization: Option<ID>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct License {
    /// The SPDX license ID of a project
    pub id: String,
    pub name: String,
    /// The URL to this license
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub url: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationLink {
    /// The donation platform's ID
    pub id: String,
    pub platform: String,
    /// A link to the donation platform and user
    pub url: Url,
}

/// An image that have been uploaded to a project's gallery
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalleryItem {
    pub url: Url,
    pub raw_url: Url,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: UtcTime,
    /// The order of the gallery image.
    /// Gallery images are sorted by this field and then alphabetically by title.
    pub ordering: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectDependencies {
    pub projects: Vec<Project>,
    pub versions: Vec<version::Version>,
}

/// Fields to edit on the projects specified
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditMultipleProjectsBody {
    /// Set all of the categories to the categories specified here
    pub categories: Vec<String>,
    /// Add all of the categories specified here
    pub add_categories: Vec<String>,
    /// Remove all of the categories specified here
    pub remove_categories: Vec<String>,
    /// Set all of the additional categories to the categories specified here
    pub additional_categories: Vec<String>,
    /// Add all of the additional categories specified here
    pub add_additional_categories: Vec<String>,
    /// Remove all of the additional categories specified here
    pub remove_additional_categories: Vec<String>,
    /// Set all of the donation links to the donation links specified here
    pub donation_urls: Vec<DonationLink>,
    /// Add all of the donation links specified here
    pub add_donation_urls: Vec<DonationLink>,
    /// Remove all of the donation links specified here
    pub remove_donation_urls: Vec<DonationLink>,
    /// A link to where to submit bugs or issues with the projects
    pub issues_url: Option<String>,
    /// A link to the source code of the projects
    pub source_url: Option<String>,
    /// A link to the projects' wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// An optional invite link to the projects' discord
    pub discord_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Approved,
    Archived,
    /// A moderator's message should be available in the project struct
    Rejected,
    Draft,
    /// The project has been approved and is publicly accessible, but will not show up in search results
    Unlisted,
    /// The project has been submitted for approval and is being reviewed
    Processing,
    Withheld,
    /// The project's status has been scheduled to change.
    /// Check the project's `requested_status` for more information.
    Scheduled,
    Private,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
    Approved,
    Archived,
    Unlisted,
    Private,
    Draft,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum MonetizationStatus {
    Monetized,
    Demonetized,
    ForceDemonetized,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSupportRange {
    /// The mod is required on this side to function
    Required,
    /// The mod is not required on this side to function.
    /// However, functionality might be enhanced if it is present.
    Optional,
    /// The mod will not run on this side
    Unsupported,
    /// It is unknown if the project will run on this side
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    /// WARNING: Can be a mod, plugin, or data pack
    ///
    /// You will have to read the loaders to get more specific information.
    Project,
    Mod,
    Shader,
    Plugin,
    Modpack,
    Datapack,
    ResourcePack,
}

/// File extensions for images
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFileExt {
    /// [Portable Network Graphics](https://en.wikipedia.org/wiki/PNG)
    PNG,
    /// [Joint Photographic Experts Group](https://en.wikipedia.org/wiki/JPEG)
    JPG,
    /// [Joint Photographic Experts Group](https://en.wikipedia.org/wiki/JPEG)
    JPEG,
    /// [Bitmap](https://en.wikipedia.org/wiki/BMP_file_format)
    BMP,
    /// [Graphics Interchange Format](https://en.wikipedia.org/wiki/GIF)
    GIF,
    /// [Web Picture](https://en.wikipedia.org/wiki/WebP)
    WebP,
    /// [Scalable Vector Graphics](https://en.wikipedia.org/wiki/SVG)
    SVG,
    /// [Scalable Vector Graphics](https://en.wikipedia.org/wiki/SVG#Compression) (gZip compressed)
    SVGZ,
    /// [Silicon Graphics Image](https://en.wikipedia.org/wiki/Silicon_Graphics_Image)
    RGB,
}

impl std::fmt::Display for ImageFileExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
