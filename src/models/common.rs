#[derive(Debug, Clone)]
pub struct Arxiv {
    id: String,
    title: String,
    summary: String,
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
