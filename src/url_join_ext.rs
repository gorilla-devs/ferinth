use reqwest::Url;

pub trait UrlJoinExt {
    fn join_all<S: Into<String>>(&self, paths: Vec<S>) -> Self;
}

impl UrlJoinExt for Url {
    /// Join all the `paths` provided
    ///
    /// Example:
    /// ```ignore
    /// # use reqwest::Url;
    /// assert_eq!(
    ///     Url::parse("https://example.com/").unwrap().join_all(&["join_all", "example", "path"]),
    ///     Url::parse("https://example.com/join_all/example/path").unwrap()
    /// )
    /// ```
    fn join_all<S: Into<String>>(&self, mut paths: Vec<S>) -> Self {
        let mut url = self.clone();
        let last = paths.pop().unwrap();
        for segment in paths {
            let mut segment = segment.into();
            segment.push('/');
            url = url.join(&segment).unwrap();
        }
        url.join(&last.into()).unwrap()
    }
}
