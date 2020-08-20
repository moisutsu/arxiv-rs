#[derive(Debug, Clone)]
pub struct Arxiv {
    pub id: String,
    pub title: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct ArxivQuery {
    pub base_url: String,
    pub search_query: String,
}

#[derive(Debug, Clone)]
pub struct ArxivQueryBuilder {
    pub base_url: String,
    pub search_query: String,
}
