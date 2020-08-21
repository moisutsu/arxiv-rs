use arxiv::{Arxiv, ArxivQueryBuilder};

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    let query = ArxivQueryBuilder::new().search_query("cat:cs.CL").build();
    let arxivs = Arxiv::fetch_data(query).await?;
    for i in 0..arxivs.len() {
        println!("{:?}", arxivs[i]);
    }
    Ok(())
}
