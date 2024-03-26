//! Extensions to `url::Url` to make it generally easier to use

use serde::Serialize;
use url::Url;

pub trait UrlJoinAll {
    /// [Url::join] all the provided `segments`
    fn join_all(&self, segments: Vec<impl Into<String>>) -> Self;
}

impl UrlJoinAll for Url {
    fn join_all(&self, mut segments: Vec<impl Into<String>>) -> Self {
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

pub trait UrlWithQuery: Sized {
    type SerialiseResult<T>;

    /// Add the `name` and `value` query to `self` and return it
    fn with_query(self, name: impl AsRef<str>, value: impl ToString) -> Self;

    /// Serialise and add the `name` and `value` query to `self` and return it
    fn with_query_json(
        self,
        name: impl AsRef<str>,
        value: impl Serialize,
    ) -> Self::SerialiseResult<Self>;
}

impl UrlWithQuery for Url {
    type SerialiseResult<T> = serde_json::Result<T>;

    fn with_query(mut self, name: impl AsRef<str>, value: impl ToString) -> Self {
        self.query_pairs_mut()
            .append_pair(name.as_ref(), &value.to_string());
        self
    }

    fn with_query_json(
        mut self,
        name: impl AsRef<str>,
        value: impl Serialize,
    ) -> Self::SerialiseResult<Self> {
        self.query_pairs_mut()
            .append_pair(name.as_ref(), &serde_json::to_string(&value)?);
        Ok(self)
    }
}
