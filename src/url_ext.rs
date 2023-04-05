//! Extensions to `url::Url` to make it generally easier to use

use std::borrow::Borrow;
use url::Url;

pub trait UrlJoinAll {
    /// [Url::join] all the provided `segments`
    fn join_all<S: Into<String>>(&self, segments: Vec<S>) -> Self;
}

impl UrlJoinAll for Url {
    fn join_all<S: Into<String>>(&self, mut segments: Vec<S>) -> Self {
        let mut url = self.clone();
        let last = segments.pop().expect("`segments` is empty");
        for segment in segments {
            let mut segment = segment.into();
            segment.push('/');
            url = url.join(&segment).expect("Invalid URL segment");
        }
        url.join(&last.into()).expect("Invalid URL segment")
    }
}

pub trait UrlWithQuery {
    /// Add the provided `queries` to `self` and return `self`
    fn with_query<I, K, V>(self, queries: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>;
}

impl UrlWithQuery for Url {
    fn with_query<I, K, V>(mut self, queries: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.query_pairs_mut().extend_pairs(queries);
        self
    }
}
