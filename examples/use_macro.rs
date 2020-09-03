use anyhow::Result;
use arxiv::query;

#[async_std::main]
async fn main() -> Result<()> {
    let query = query!(
        search_query = "cat:cs.CL",
        start = 0,
        max_results = 5,
        sort_by = "submittedDate",
        sort_order = "descending"
    );
    let arxivs = arxiv::fetch_arxivs(query).await?;
    for arxiv in arxivs {
        println!("{:?}", arxiv);
    }
    Ok(())
}
