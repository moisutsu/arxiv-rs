use crate::{ArxivQuery, ArxivQueryBuilder};

impl ArxivQueryBuilder {
    pub fn new() -> Self {
        ArxivQueryBuilder {
            base_url: "http://export.arxiv.org/api/query?".to_string(),
            search_query: "".to_string(),
            id_list: "".to_string(),
            start: None,
            max_results: None,
        }
    }
    pub fn build(&self) -> ArxivQuery {
        ArxivQuery {
            base_url: self.base_url.clone(),
            search_query: self.search_query.clone(),
            id_list: self.id_list.clone(),
            start: self.start,
            max_results: self.max_results,
        }
    }
    pub fn search_query(&self, search_query: &str) -> Self {
        ArxivQueryBuilder {
            search_query: search_query.to_string(),
            ..self.clone()
        }
    }
    pub fn id_list(&self, id_list: &str) -> Self {
        ArxivQueryBuilder {
            id_list: id_list.to_string(),
            ..self.clone()
        }
    }
    pub fn start(&self, start: i32) -> Self {
        ArxivQueryBuilder {
            start: Some(start),
            ..self.clone()
        }
    }
    pub fn max_results(&self, max_results: i32) -> Self {
        ArxivQueryBuilder {
            max_results: Some(max_results),
            ..self.clone()
        }
    }
}

impl std::string::ToString for ArxivQuery {
    fn to_string(&self) -> String {
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
        format!("{}{}", self.base_url, querys.join("&"))
    }
}
