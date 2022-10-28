use crate::structures::project::ProjectType;

struct Facet {
    key: String,
    value: String,
}

impl Facet {
    pub fn new<T: Into<String>>(key: T, value: T) -> Facet {
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

impl Into<String> for &Facet {
    fn into(self) -> String {
        format!("{}:{}", self.key, self.value)
    }
}

struct FacetBuilder {
    stack: Vec<Vec<Facet>>,
    cur: Vec<Facet>,
}

impl FacetBuilder {
    /// Creates a new [`FacetBuilder`]
    /// # Example
    /// ```rs
    /// // Search for 1.18.2 mods
    /// let search = FacetBuilder::new(ModrinthFacet::version("1.18.2"))
    /// ```
    pub fn new(facet: Facet) -> FacetBuilder {
        FacetBuilder {
            stack: vec![],
            cur: vec![facet],
        }
    }

    /// Adds an AND expression to the current builder.  
    /// # Example
    /// ```rs
    /// // Search for 1.18.2 AND quilt mods
    /// let search = FacetBuilder::new(ModrinthFacet::version("1.18.2"))    
    ///     .and(ModrinthFacet::category("quilt"))
    ///     .build();
    /// // Builds to : [["version:1.18.2","category:quilt"]]
    /// ```
    pub fn and(mut self, facet: Facet) -> FacetBuilder {
        self.cur.push(facet);
        self
    }

    /// Adds an OR expression to the current builder.  
    /// # Example
    /// ```rs
    /// // Search for 1.18.2 AND quilt OR fabric mods
    /// let search = FacetBuilder::new(ModrinthFacet::version("1.18.2"))    
    ///     .and(ModrinthFacet::category("quilt"))
    ///     .or(ModrinthFacet::category("fabric"))
    ///     .build();
    /// // Builds to : [["version:1.18.2","category:quilt"], ["category:fabric"]]
    /// ```
    pub fn or(mut self, facet: Facet) -> FacetBuilder {
        self.stack.push(self.cur);
        self.cur = vec![facet];
        self
    }

    /// Serializes the [`FacetBuilder`] into the structure required by Modrinth (Meilisearch)
    pub fn build(mut self) -> Vec<Vec<String>> {
        self.stack.push(self.cur);
        self.stack
            .iter()
            .map(|x| x.iter().map(|z| z.into()).collect())
            .collect()
    }
}

struct ModrinthFacet {}

impl ModrinthFacet {
    /// Creates a valid Modrinth [`Facet`]  
    /// # Arguments
    /// * `version` - The minecraft version to filter the results from
    pub fn version<T: Into<String>>(version: T) -> Facet {
        Facet {
            key: "versions".to_string(),
            value: version.into(),
        }
    }
    /// Creates a valid Modrinth [`Facet`]  
    /// # Arguments
    /// * `project_type` - The [`ProjectType`] to filter the results from
    pub fn project_type(project_type: ProjectType) -> Facet {
        Facet {
            key: "project_type".to_string(),
            value: format!("{:?}", project_type).to_lowercase(),
        }
    }
    /// Creates a valid Modrinth [`Facet`]  
    /// # Arguments
    /// * `license` - The license ID (e.g. mit, arr) to filter the results from
    pub fn license<T: Into<String>>(license: T) -> Facet {
        Facet {
            key: "license".to_string(),
            value: license.into(),
        }
    }
    /// Creates a valid Modrinth [`Facet`]  
    /// # Arguments
    /// * `category` - The loader or category to filter the results from
    pub fn category<T: Into<String>>(category: T) -> Facet {
        Facet {
            key: "categories".to_string(),
            value: category.into(),
        }
    }
}
