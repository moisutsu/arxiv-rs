<h1 align="center">Welcome to arxiv-rs 👋</h1>

[![Crates.io](https://img.shields.io/crates/v/arxiv-rs.svg)](https://crates.io/crates/arxiv-rs)
[![Documentation](https://docs.rs/arxiv-rs/badge.svg)](https://docs.rs/arxiv-rs)
![License](https://img.shields.io/crates/l/arxiv-rs.svg)

This is a wrapper crate for the arXiv API

## Usage

Add dependencies to `Cargo.toml`

```toml
[dependencies]
arxiv-rs = "0.1.1"
async-std = { version = "1.6.0", features = ["attributes"] }
anyhow = "1.0.32"
```

Fetch the paper information and save it as a pdf

```rust
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
        arxiv.fetch_pdf(&arxiv.title).await?;
    }
    Ok(())
}
```

You can easily build the query using the macro

```rust
use arxiv::query;

let query = query!(
    search_query = "cat:cs.CL",
    start = 0,
    max_results = 5,
    sort_by = "submittedDate",
    sort_order = "descending"
);
```

## Show your support

Give a ⭐️ if this project helped you!

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
