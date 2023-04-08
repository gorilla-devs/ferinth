//! Extensions to `url::Url` to make it generally easier to use

use serde::Serialize;
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

pub trait UrlWithQuery
where
    Self: Sized,
{
    /// Add the provided `query` to `self` and return `self`
    fn with_query<V: ToString>(self, name: &str, value: V) -> Self;

    /// Add the provided `query` to `self` and return `self`
    fn with_query_json<V: Serialize>(self, name: &str, value: V) -> serde_json::Result<Self>;
}

impl UrlWithQuery for Url {
    fn with_query<V: ToString>(mut self, name: &str, value: V) -> Self {
        self.query_pairs_mut().append_pair(name, &value.to_string());
        self
    }

    fn with_query_json<V: Serialize>(mut self, name: &str, value: V) -> serde_json::Result<Self> {
        self.query_pairs_mut()
            .append_pair(name, &serde_json::to_string(&value)?);
        Ok(self)
    }
}
