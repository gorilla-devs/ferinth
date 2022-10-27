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

struct FacetBuilder {
    stack: Vec<Vec<Facet>>,
    cur: Vec<Facet>,
}

impl FacetBuilder {
    fn new(facet: Facet) -> FacetBuilder {
        FacetBuilder {
            stack: vec![],
            cur: vec![facet],
        }
    }

    fn and(mut self, facet: Facet) -> FacetBuilder {
        self.cur.push(facet);
        self
    }

    fn or(mut self, facet: Facet) -> FacetBuilder {
        self.stack.push(self.cur);
        self.cur = vec![facet];
        self
    }

    fn build(mut self) -> Vec<Vec<String>> {
        self.stack.push(self.cur);
        self.stack
            .iter()
            .map(|x| x.iter().map(|z| z.into()))
            .collect()
    }
}
