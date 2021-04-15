/// A structure that stores the paper information.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Arxiv {
    pub id: String,
    pub updated: String,
    pub published: String,
    pub title: String,
    pub summary: String,
    pub authors: Vec<String>,
    pub pdf_url: String,
    pub comment: Option<String>,
}

/// A structure that stores the query information.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArxivQuery {
    pub base_url: String,
    pub search_query: String,
    pub id_list: String,
    pub start: Option<i32>,
    pub max_results: Option<i32>,
    pub sort_by: String,
    pub sort_order: String,
}

/// A builder of ArxivQuery
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ArxivQueryBuilder {
    pub base_url: String,
    pub search_query: String,
    pub id_list: String,
    pub start: Option<i32>,
    pub max_results: Option<i32>,
    pub sort_by: String,
    pub sort_order: String,
}
