use super::*;
use structures::{search::*, Number};

impl Ferinth {
    /**
    Search for projects using `query` string, with pagination

    Limit the number of responses to `limit` projects, and offset the output by `offset` projects.
    Sort projects by `sort`, and filter projects using the given `facets`.
    In `facets`, only non-empty vectors will be added to the query paramater.
    */
    pub async fn search_paged(
        &self,
        query: &str,
        sort: &Sort,
        limit: Number,
        offset: Number,
        mut facets: Vec<Vec<Facet>>,
    ) -> Result<Response> {
        let mut url = API_BASE_URL
            .join_all(vec!["search"])
            .with_query("query", query)
            .with_query("index", sort)
            .with_query("limit", limit)
            .with_query("offset", offset);

        facets.retain(|e| !e.is_empty());
        if !facets.is_empty() {
            url = url.with_query_json("facets", facets)?
        }

        self.client.get(url).custom_send_json().await
    }

    /**
    Search for projects using `query` string

    Sort the hits by `sort`, and filter projects using the given `facets`.
    In `facets`, only non-empty lists will be added to the query paramater.

    ```rust
    # use ferinth::structures::search::{Sort, Facet};
    # #[tokio::main]
    # async fn main() -> Result<(), ferinth::Error> {
    # let modrinth = ferinth::Ferinth::default();
    // When searching for 'sodium' and filtering by Forge mods
    let results = modrinth.search(
        "sodium",
        &Sort::Downloads,
        vec![vec![ Facet::Categories("forge".into()) ]]
    ).await?;
    // Rubidium should be the result with the most downloads
    assert_eq!(&results.hits[0].slug, "rubidium");
    # Ok(()) }
    ```
    */
    pub async fn search(
        &self,
        query: &str,
        sort: &Sort,
        mut facets: Vec<Vec<Facet>>,
    ) -> Result<Response> {
        let mut url = API_BASE_URL
            .join_all(vec!["search"])
            .with_query("query", query)
            .with_query("index", sort);

        facets.retain(|e| !e.is_empty());
        if !facets.is_empty() {
            url = url.with_query_json("facets", facets)?
        }

        self.client.get(url).custom_send_json().await
    }
}
