use crate::structures::project::ProjectType;

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

struct ModrinthFacet {}

impl ModrinthFacet {
    /// Creates a valid Modrinth [`Facet`]  
    /// # Arguments
    /// * `version` - The minecraft version to filter the results from
    fn version<T: Into<String>>(version: T) -> Facet {
        Facet {
            key: "versions".to_string(),
            value: version.into(),
        }
    }
    /// Creates a valid Modrinth [`Facet`]  
    /// # Arguments
    /// * `project_type` - The [`ProjectType`] to filter the results from
    fn project_type(project_type: ProjectType) -> Facet {
        Facet {
            key: "project_type".to_string(),
            value: format!("{:?}", project_type).to_lowercase(),
        }
    }
    /// Creates a valid Modrinth [`Facet`]  
    /// # Arguments
    /// * `license` - The license ID (e.g. mit, arr) to filter the results from
    fn license<T: Into<String>>(license: T) -> Facet {
        Facet {
            key: "license".to_string(),
            value: license.into(),
        }
    }
    /// Creates a valid Modrinth [`Facet`]  
    /// # Arguments
    /// * `category` - The loader or category to filter the results from
    fn category<T: Into<String>>(category: T) -> Facet {
        Facet {
            key: "categories".to_string(),
            value: category.into(),
        }
    }
}
