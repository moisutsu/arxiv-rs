use crate::{Arxiv, ArxivQuery};

use anyhow::{anyhow, Result};
use xml::reader::{EventReader, XmlEvent};

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

    pub async fn fetch_arxivs(query: ArxivQuery) -> Result<Vec<Arxiv>> {
        let mut response = surf::get(query.to_string())
            .await
            .map_err(|err| anyhow!(err))?;
        let body = response.body_string().await.map_err(|err| anyhow!(err))?;
        let arxivs = Arxiv::parse_data(body)?;
        Ok(arxivs)
    }

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

    fn parse_data(body: String) -> Result<Vec<Arxiv>> {
        let mut parser = EventReader::from_str(&body);
        let mut arxiv = Arxiv::new();
        let mut arxivs = Vec::new();

        'outer: loop {
            match parser.next()? {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => match &name.local_name[..] {
                    "entry" => {
                        arxiv = Arxiv::new();
                    }
                    "id" => {
                        arxiv.id = if let XmlEvent::Characters(id) = parser.next()? {
                            id
                        } else {
                            arxiv.id
                        };
                    }
                    "updated" => {
                        arxiv.updated = if let XmlEvent::Characters(updated) = parser.next()? {
                            updated
                        } else {
                            arxiv.updated
                        };
                    }
                    "published" => {
                        arxiv.published = if let XmlEvent::Characters(published) = parser.next()? {
                            published
                        } else {
                            arxiv.published
                        };
                    }
                    "title" => {
                        arxiv.title = if let XmlEvent::Characters(title) = parser.next()? {
                            title
                        } else {
                            arxiv.title
                        };
                    }
                    "summary" => {
                        arxiv.summary = if let XmlEvent::Characters(summary) = parser.next()? {
                            summary
                        } else {
                            arxiv.summary
                        };
                    }
                    "author" => {
                        parser.next()?;
                        parser.next()?;
                        if let XmlEvent::Characters(author) = parser.next()? {
                            arxiv.authors.push(author);
                        }
                    }
                    "link" => {
                        if attributes[0].value == "pdf" {
                            arxiv.pdf_url = format!(
                                "{}.pdf",
                                attributes[1].value.replacen("http", "https", 1).clone()
                            );
                        }
                    }
                    _ => (),
                },
                XmlEvent::EndElement { name } => match &name.local_name[..] {
                    "entry" => {
                        arxivs.push(arxiv.clone());
                    }
                    "feed" => {
                        break 'outer;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        Ok(arxivs)
    }
}
