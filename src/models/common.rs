#[derive(Debug, Clone)]
pub struct Arxiv {
    id: String,
    title: String,
    summary: String,
}

#[derive(Debug, Clone)]
pub struct ArxivQuery {
    base_url: String,
    search_query: String,
}

#[derive(Debug, Clone)]
pub struct ArxivQueryBuilder {
    base_url: String,
    search_query: String,
}
