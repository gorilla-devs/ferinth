//! Miscellaneous API calls
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/misc)

use super::*;
use crate::structures::misc::*;

impl Ferinth<Authenticated> {
    /**
    Submit a report to the moderators

    Valid report types can be found using [`Ferinth::list_report_types`]

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    let report = modrinth.submit_report(&ferinth::structures::misc::ReportSubmission {
        report_type: "other".to_string(),
        item_id: "XXXXXXXX".to_string(),
        item_type: ferinth::structures::misc::ReportItemType::User,
        body: "This is an example report".to_string(),
    }).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn submit_report(&self, report: &ReportSubmission) -> Result<Report> {
        check_id_slug(&[&report.item_id])?;
        self.client
            .post(API_BASE_URL.join_all(vec!["report"]))
            .json(report)
            .custom_send_json()
            .await
    }
}

impl<T> Ferinth<T> {
    /**
    Get various statistics about this Modrinth instance

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let statistics = modrinth.instance_statistics().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn instance_statistics(&self) -> Result<Statistics> {
        self.client
            .get(API_BASE_URL.join_all(vec!["statistics"]))
            .custom_send_json()
            .await
    }

    /**
    Get the Modrinth API welcome page.

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    modrinth.welcome().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn welcome(&self) -> Result<Welcome> {
        self.client
            .get(crate::BASE_URL.as_ref())
            .custom_send_json()
            .await
    }
}
