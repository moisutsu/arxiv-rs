use arxiv::{fetch_arxivs, query};

#[async_std::test]
async fn test_fetch_arxivs() {
    let query = query!(id_list = "1706.03762v5");
    let arxivs = fetch_arxivs(query).await.unwrap();
    assert_eq!(arxivs[0].title, "Attention Is All You Need")
}
