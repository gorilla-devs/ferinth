use crate::request::API_URL_BASE;
use crate::structures::search::{SearchResponse, SortingMethod};
use crate::structures::Number;
use crate::url_join_ext::UrlJoinExt;
use crate::{Ferinth, Result};

type Facets = Vec<Vec<String>>;

impl Ferinth {
    /// Searches a project based on the `query` provided with
    /// support for additional filtering
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # use ferinth::structures::search::SortingMethod;
    ///
    /// let modrinth = ferinth::Ferinth::default();
    /// let sodium_result = modrinth.search("sodium".to_string(),vec![], SortingMethod::Relevance, None, None, None, None).await?;
    /// assert_eq!(
    ///     sodium_result.hits.get(0)?.title,
    ///     "Sodium",
    /// );
    /// # Ok(()) }
    /// ```
    ///
    /// You can also use [`ModrinthFacet`](crate::utils::facet::ModrinthFacet) and [`FacetBuilder`](crate::utils::facet::FacetBuilder) to filter by version/loader using facets in a type-safe manner.
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # use ferinth::structures::search::SortingMethod;
    /// # use ferinth::utils::facet::{FacetBuilder, ModrinthFacet};
    ///
    /// let modrinth = ferinth::Ferinth::default();
    ///
    /// let facets = FacetBuilder::new(ModrinthFacet::version("1.19.2"))
    ///     .and(ModrinthFacet::category("quilt")).build();
    ///
    /// let ok_zoomer_response = modrinth.search("ok zoomer".to_string(), facets, SortingMethod::Relevance, None, None, None, None).await?;
    /// assert_eq!(
    ///     ok_zoomer_response.hits.get(0)?.title,
    ///     "Ok Zoomer",
    /// );
    /// # Ok(()) }
    /// ```
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
        let facet_request = if facets.is_empty() { None } else { Some(serde_json::to_string(&facets)?) };
        let offset_request = if offset.is_none() { None } else { Some(offset.unwrap().to_string()) };
        let limit_request = if limit.is_none() { None } else { Some(limit.unwrap().to_string()) };

        self.get_with_optional_query(
            API_URL_BASE.join_all(vec!["search"]),
            &[
                ("query", Some(query)),
                ("facets", facet_request),
                ("index", Some(format!("{:?}",index).to_lowercase())),
                ("offset", offset_request),
                ("limit", limit_request),
                ("filters", filters),
                ("version", version),
            ],
        )
        .await
    }
}
