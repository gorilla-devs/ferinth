use crate::request::API_URL_BASE;
use crate::structures::search::{SearchResponse, SortingMethod};
use crate::structures::Number;
use crate::url_join_ext::UrlJoinExt;
use crate::{Ferinth, Result};

type Facets = Vec<Vec<String>>;

impl Ferinth {
    pub async fn search(
        &self,
        query: String,
        facets: Facets,
        index: SortingMethod,
        offset: Option<Number>,
        limit: Option<Number>,
        filters: Option<String>,
        version: Option<String>,
    ) -> Result<SearchResponse> {
        self.get_with_query(
            API_URL_BASE.join_all(vec!["search"]),
            &[
                ("query", query),
                ("facets", serde_json::to_string(&facets)?),
                ("index", serde_json::to_string(&index)?),
                ("offset", serde_json::to_string(&offset.unwrap_or(0))?),
                ("limit", serde_json::to_string(&limit.unwrap_or(10))?),
                ("filters", filters.unwrap_or(String::default())),
                ("version", version.unwrap_or(String::default())),
            ],
        )
        .await
    }
}
