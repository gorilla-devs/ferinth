use super::*;
use crate::structures::{search::*, Number};

impl Ferinth {
    /// Search for projects using `query` string, with pagination
    ///
    /// Limit the number of responses to `limit` projects, and offset the output by `offset` projects.
    /// Sort mods by `sort`, and filter mods using the given `facets`.
    pub async fn search_paged(
        &self,
        query: &str,
        sort: &Sort,
        limit: &Number,
        offset: &Number,
        facets: &[&[Facet]],
    ) -> Result<Response> {
        self.get_with_query(
            API_URL_BASE.join("search").unwrap(),
            &[
                ("query", query),
                ("index", &sort.to_string()),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
                ("facets", &serde_json::to_string(facets)?),
            ],
        )
        .await
    }

    /// Search for projects using `query` string
    ///
    /// Sort mods by `sort`, and filter mods using the given `facets`.
    ///
    /// Example:
    /// ```rust
    /// # use ferinth::structures::search::Sort;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let results = modrinth.search("sodium", &Sort::Relevance, &[]).await?;
    /// // The Sodium mod should be the most relevant response when `"sodium"` is searched
    /// assert_eq!(&results.hits[0].id, "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn search(&self, query: &str, sort: &Sort, facets: &[&[Facet]]) -> Result<Response> {
        self.get_with_query(
            API_URL_BASE.join("search").unwrap(),
            &[
                ("query", query),
                ("sort", &sort.to_string()),
                ("facets", &serde_json::to_string(facets)?),
            ],
        )
        .await
    }
}
