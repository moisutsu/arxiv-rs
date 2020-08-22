#[derive(Debug, Clone)]
pub struct Arxiv {
    pub id: String,
    pub updated: String,
    pub published: String,
    pub title: String,
    pub summary: String,
    pub authors: Vec<String>,
    pub pdf_url: String,
}

#[derive(Debug, Clone)]
pub struct ArxivQuery {
    pub base_url: String,
    pub search_query: String,
    pub id_list: String,
    pub start: Option<i32>,
    pub max_results: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ArxivQueryBuilder {
    pub base_url: String,
    pub search_query: String,
    pub id_list: String,
    pub start: Option<i32>,
    pub max_results: Option<i32>,
}
