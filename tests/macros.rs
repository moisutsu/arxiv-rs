use arxiv::{query, ArxivQuery};

#[test]
fn query_macro() {
    let query_1 = ArxivQuery {
        base_url: "http://export.arxiv.org/api/query?".to_string(),
        search_query: "search_query".to_string(),
        id_list: "id_list".to_string(),
        start: Some(3),
        max_results: Some(5),
        sort_by: "sort_by".to_string(),
        sort_order: "sort_order".to_string(),
    };
    let query_2 = query!(
        search_query = "search_query",
        id_list = "id_list",
        start = 3,
        max_results = 5,
        sort_by = "sort_by",
        sort_order = "sort_order"
    );
    assert_eq!(query_1, query_2);
}
