use crate::Arxiv;

use anyhow::{anyhow, Result};

use std::fs;
use std::io::Write;

impl Arxiv {
    pub fn new() -> Self {
        Arxiv {
            id: "".to_string(),
            updated: "".to_string(),
            published: "".to_string(),
            title: "".to_string(),
            summary: "".to_string(),
            authors: Vec::new(),
            pdf_url: "".to_string(),
        }
    }

    /// Save the paper as a pdf from the information stored by the structure.
    pub async fn fetch_pdf(&self, out_path: &str) -> Result<()> {
        let mut response = surf::get(&self.pdf_url).await.map_err(|err| anyhow!(err))?;
        let body = response.body_bytes().await.map_err(|err| anyhow!(err))?;
        let out_path = if out_path.ends_with(".pdf") {
            out_path.to_string()
        } else {
            format!("{}.pdf", out_path)
        };
        let mut file = fs::File::create(out_path)?;
        file.write_all(&body)?;
        Ok(())
    }
}
