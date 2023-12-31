use std::ops::Deref;

// type aliases for ease of reading
pub type Fields = Vec<String>;
pub type Exclude = Vec<String>;
pub type Limit = u128;
pub type Offset = u128;

// TODO: write typesafe version later
pub type WhereRaw = String;

#[derive(Debug, Default)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

#[derive(Debug, Default)]
pub struct Sort {
    pub field: String,
    pub direction: SortOrder,
}

#[derive(Debug, Default)]
pub struct Search {
    pub column: Option<String>,
    pub query: String,
}

#[derive(Debug, Default)]
pub struct Query {
    pub fields: Fields,
    pub exclude: Exclude,
    pub limit: Option<Limit>,
    pub offset: Option<Offset>,
    pub where_raw: Option<WhereRaw>,
    pub sort: Option<Sort>,
    pub search: Option<Search>,
}

impl Query {
    pub fn New() -> Self {
        Self::default()
    }

    pub fn field(mut self, field: &str) -> Self {
        self.fields.push(field.to_owned());
        self
    }

    pub fn exclude(mut self, exclude_field: &str) -> Self {
        self.exclude.push(exclude_field.to_owned());
        self
    }

    pub fn limit(mut self, new_limit: u128) -> Self {
        self.limit = Some(new_limit);
        self
    }

    pub fn offset(mut self, new_offset: u128) -> Self {
        self.offset = Some(new_offset);
        self
    }

    pub fn add_where(mut self, condition: &str) -> Self {
        self.where_raw = Some(condition.to_owned());
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn search(mut self, search: Search) -> Self {
        self.search = Some(search);
        self
    }

    pub fn serialize(&self) -> String {
        let mut serialized = String::new();

        if !self.fields.is_empty() {
            let fields = self.fields.join(",");
            let fields = format!("fields {};", fields);
            serialized.push_str(&fields)
        }

        if !self.exclude.is_empty() {
            let exclude = self.exclude.join(",");
            let exclude = format!("exclude {};", exclude);
            serialized.push_str(&exclude)
        }

        if let Some(where_raw) = &self.where_raw {
            let where_raw = format!("where {};", where_raw);
            serialized.push_str(&where_raw)
        }

        if let Some(limit) = &self.limit {
            let limit = format!("limit {};", limit);
            serialized.push_str(&limit);
        }

        if let Some(offset) = &self.offset {
            let offset = format!("offset {};", offset);
            serialized.push_str(&offset)
        }

        if let Some(sort) = &self.sort {
            let direction = match sort.direction {
                SortOrder::Ascending => "asc",
                SortOrder::Descending => "desc",
            };
            let sort = format!("sort {} {};", sort.field, direction);
            serialized.push_str(&sort)
        }

        if let Some(search) = &self.search {
            let column = search.column.clone().unwrap_or_default();
            let search = format!(r##"search {} "{}";"##, column, search.query);
            serialized.push_str(&search)
        }

        serialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = Query::New()
            .field("a")
            .field("b")
            .field("c")
            .exclude("d")
            .exclude("e")
            .add_where("b.count >= 14 & a != n")
            .limit(8)
            .offset(2)
            .sort(Sort {
                field: "b.count".to_owned(),
                direction: SortOrder::Descending,
            })
            .search(Search {
                column: None,
                query: "test".to_owned(),
            })
            .serialize();
        panic!("{}", actual)
    }
}
