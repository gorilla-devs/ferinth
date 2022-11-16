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
        let facetRequest = if facets.is_empty() { None } else { Some(serde_json::to_string(&facets)?) };
        let offsetRequest = if offset.is_none() { None } else { Some(offset.unwrap().to_string()) };
        let limitRequest = if limit.is_none() { None } else { Some(limit.unwrap().to_string()) };

        self.get_with_optional_query(
            API_URL_BASE.join_all(vec!["search"]),
            &[
                ("query", Some(query)),
                ("facets", facetRequest),
                ("index", Some(format!("{:?}",index).to_lowercase())),
                ("offset",offsetRequest),
                ("limit", limitRequest),
                ("filters", filters),
                ("version", version),
            ],
        )
        .await
    }
}
