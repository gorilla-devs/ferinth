struct Facet {
    key: String,
    value: String,
}

impl Facet {
    fn new<T: Into<String>>(key: T, value: T) -> Facet {
        Facet {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Into<String> for Facet {
    fn into(self) -> String {
        format!("{}:{}", self.key, self.value)
    }
}
