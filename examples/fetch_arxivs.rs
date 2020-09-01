use anyhow::Result;
use arxiv::ArxivQueryBuilder;

#[async_std::main]
async fn main() -> Result<()> {
    let query = ArxivQueryBuilder::new()
        .search_query("cat:cs.CL")
        .start(0)
        .max_results(5)
        .sort_by("submittedDate")
        .sort_order("descending")
        .build();
    let arxivs = arxiv::fetch_arxivs(query).await?;
    for arxiv in arxivs {
        println!("{:?}", arxiv);
    }
    Ok(())
}
