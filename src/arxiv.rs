use crate::Arxiv;

impl Arxiv {
    pub fn new() -> Self {
        Arxiv {
            id: "".to_string(),
            title: "".to_string(),
            summary: "".to_string(),
        }
    }
}
