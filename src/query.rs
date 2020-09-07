use crate::{ArxivQuery, ArxivQueryBuilder};

impl ArxivQueryBuilder {
    pub fn new() -> Self {
        ArxivQueryBuilder {
            base_url: "http://export.arxiv.org/api/query?".to_string(),
            search_query: "".to_string(),
            id_list: "".to_string(),
            start: None,
            max_results: None,
            sort_by: "".to_string(),
            sort_order: "".to_string(),
        }
    }
    /// Build ArxivQuery from ArxivQueryBuilder.
    pub fn build(&self) -> ArxivQuery {
        ArxivQuery {
            base_url: self.base_url.clone(),
            search_query: self.search_query.clone(),
            id_list: self.id_list.clone(),
            start: self.start,
            max_results: self.max_results,
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
        }
    }
    /// Store the argument value in search_query.
    pub fn search_query(&self, search_query: &str) -> Self {
        ArxivQueryBuilder {
            search_query: search_query.to_string(),
            ..self.clone()
        }
    }
    /// Store the argument value in id_list.
    pub fn id_list(&self, id_list: &str) -> Self {
        ArxivQueryBuilder {
            id_list: id_list.to_string(),
            ..self.clone()
        }
    }
    /// Store the argument value in start.
    pub fn start(&self, start: i32) -> Self {
        ArxivQueryBuilder {
            start: Some(start),
            ..self.clone()
        }
    }
    /// Store the argument value in max_results.
    pub fn max_results(&self, max_results: i32) -> Self {
        ArxivQueryBuilder {
            max_results: Some(max_results),
            ..self.clone()
        }
    }
    /// Store the argument value in sort_by.
    pub fn sort_by(&self, sort_by: &str) -> Self {
        ArxivQueryBuilder {
            sort_by: sort_by.to_string(),
            ..self.clone()
        }
    }
    /// Store the argument value in sort_order.
    pub fn sort_order(&self, sort_order: &str) -> Self {
        ArxivQueryBuilder {
            sort_order: sort_order.to_string(),
            ..self.clone()
        }
    }
}

impl ArxivQuery {
    /// Generate a URL string.
    pub fn to_url(&self) -> String {
        let mut querys = Vec::new();
        if &self.search_query != "" {
            querys.push(format!("search_query={}", self.search_query));
        }
        if &self.id_list != "" {
            querys.push(format!("id_list={}", self.id_list));
        }
        if let Some(start) = self.start {
            querys.push(format!("start={}", start));
        }
        if let Some(max_results) = self.max_results {
            querys.push(format!("max_results={}", max_results));
        }
        if &self.sort_by != "" {
            querys.push(format!("sortBy={}", self.sort_by));
        }
        if &self.sort_order != "" {
            querys.push(format!("sortOrder={}", self.sort_order));
        }
        format!("{}{}", self.base_url, querys.join("&"))
    }
}
