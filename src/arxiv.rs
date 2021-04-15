use crate::Arxiv;

use anyhow::Result;

use std::fs;
use std::io::Write;

impl Arxiv {
    pub fn new() -> Self {
        Arxiv::default()
    }

    /// Save the paper as a pdf from the information stored by the structure.
    pub async fn fetch_pdf(&self, out_path: &str) -> Result<()> {
        let body = reqwest::get(&self.pdf_url).await?.bytes().await?;
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
