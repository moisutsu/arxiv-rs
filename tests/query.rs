use arxiv::{ArxivQuery, ArxivQueryBuilder};

#[test]
fn builder() {
    let query = ArxivQuery {
        base_url: "http://export.arxiv.org/api/query?".to_string(),
        search_query: "search_query".to_string(),
        id_list: "id_list".to_string(),
        start: Some(3),
        max_results: Some(5),
        sort_by: "sort_by".to_string(),
        sort_order: "sort_order".to_string(),
    };
    let builder_query = ArxivQueryBuilder::new()
        .search_query("search_query")
        .id_list("id_list")
        .start(3)
        .max_results(5)
        .sort_by("sort_by")
        .sort_order("sort_order")
        .build();
    assert_eq!(query, builder_query);
}

#[test]
fn to_url() {
    let query = ArxivQuery {
        base_url: "http://export.arxiv.org/api/query?".to_string(),
        search_query: "search_query".to_string(),
        id_list: "id_list".to_string(),
        start: Some(3),
        max_results: Some(5),
        sort_by: "sort_by".to_string(),
        sort_order: "sort_order".to_string(),
    };
    assert_eq!(query.to_url(), "http://export.arxiv.org/api/query?search_query=search_query&id_list=id_list&start=3&max_results=5&sortBy=sort_by&sortOrder=sort_order");
}
