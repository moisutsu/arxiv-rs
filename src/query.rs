use crate::{ArxivQuery, ArxivQueryBuilder};

impl ArxivQueryBuilder {
    pub fn new() -> Self {
        ArxivQueryBuilder {
            base_url: "http://export.arxiv.org/api/query?".to_string(),
            search_query: "".to_string(),
        }
    }
    pub fn build(&self) -> ArxivQuery {
        ArxivQuery {
            base_url: self.base_url.clone(),
            search_query: self.search_query.clone(),
        }
    }
    pub fn search_query(&mut self, search_query: &str) -> Self {
        ArxivQueryBuilder {
            base_url: self.base_url.clone(),
            search_query: search_query.to_string(),
        }
    }
}

impl ArxivQuery {
    pub fn to_string(&self) -> String {
        format!("{}search_query={}", self.base_url, self.search_query)
    }
}
