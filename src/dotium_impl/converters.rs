use crate::structures::*;
use dotium::{
    self,
    project::{self, Author},
    request::{Asset, AssetHash},
    version,
};

pub fn project(raw: project_structs::Project) -> project::Project {
    project::Project {
        id: raw.id,
        slug: raw.slug,
        project_type: raw.project_type,
        name: raw.title,
        description: raw.description,
        links: project::Links {
            github: raw.source_url,
            issues: raw.issues_url,
            wiki: raw.wiki_url,
            discord: raw.discord_url,
            donations: raw.donation_urls,
        },
        requirements: project::ProjectRequirements {
            server: raw.server_side,
            client: raw.client_side,
        },
        categories: raw.categories,
        downloads: raw.downloads,
        followers: raw.followers,
        icon: raw.icon_url.and_then(|i| Some(Asset::by_url(i))),
        status: raw.status,
        published: raw.published,
        updated: raw.updated,
        created: raw.published,
        gallery: raw
            .gallery
            .into_iter()
            .map(|i| Asset {
                url: i.url,
                name: i.title,
                description: i.description,
                headers: std::collections::HashMap::new(),
                request_type: dotium::request::RequestType::GET,
                hash: None,
                size: None,
            })
            .collect(),
        allows_distribution: raw.license.short != "custom".to_string()
            && raw.license.short != "arr".to_string(),
        team_id: raw.team,
    }
}

pub fn version(raw: version_structs::Version) -> version::Version {
    version::Version {
        id: raw.id,
        name: raw.name,
        identifier: raw.version_number,
        project_id: raw.project_id,
        files: raw
            .files
            .into_iter()
            .map(|f| Asset {
                url: f.url,
                name: Some(f.filename),
                description: None,
                headers: std::collections::HashMap::new(),
                request_type: dotium::request::RequestType::GET,
                hash: Some(AssetHash {
                    hash: f.hashes.sha512.unwrap(), // Modrinth generated hashes for old versions
                    algorithm: dotium::request::HashAlgorithm::Sha512,
                }),
                size: Some(f.size),
            })
            .collect(),
        downloads: raw.downloads,
        loaders: raw.loaders,
        game_versions: raw.game_versions,
        published: raw.date_published,
        version_type: raw.version_type,
        dependencies: raw
            .dependencies
            .into_iter()
            .map(|d| version::VersionDependency {
                project_id: d.project_id,
                version: d.version_id,
                dependency_type: dependency_type(d.dependency_type),
            })
            .collect(),
    }
}

pub fn author(raw: user_structs::User) -> project::Author {
    Author {
        username: raw.username.clone(),
        name: raw.name.unwrap_or(raw.username),
        id: raw.id,
        avatar_url: Some(Asset::by_url(raw.avatar_url)),
    }
}

pub fn dependency_type(raw: version_structs::DependencyType) -> version::DependencyType {
    match raw {
        version_structs::DependencyType::Required => version::DependencyType::ReqquiredDependency,
        version_structs::DependencyType::Optional => version::DependencyType::OptionalDependency,
        version_structs::DependencyType::Incompatible => version::DependencyType::Incompatible,
    }
}
