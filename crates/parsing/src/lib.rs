use std::collections::HashMap;

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ParsingRequest {
    model: String,
    document: Document,
    include_image_base64: bool,
}

#[derive(Debug, Serialize)]
struct Document {
    r#type: String,
    document_url: String,
}

#[derive(Debug, Deserialize)]
struct ParsingResponse {
    pages: Vec<Page>,
}

#[derive(Debug, Deserialize)]
struct Page {
    index: String,
    markdown: String,
}

pub async fn parse_pdf(document_url: &str, api_key: &str) -> Result<HashMap<i32, String>, Error> {
    let url = "https://api.mistral.ai/v1/ocr";

    let request = ParsingRequest {
        model: "models/gemini-embedding-exp-03-07".to_string(),
        document: Document {
            r#type: "document_url".to_string(),
            document_url: document_url.to_string(),
        },
        include_image_base64: false,
    };

    let client = Client::new();
    let response = client
        .post(url)
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await?
        .json::<ParsingResponse>()
        .await?;

    // Return a hashmap of page index to markdown content
    Ok(response
        .pages
        .iter()
        .map(|page| {
            let index = page.index.parse::<i32>().unwrap();
            (index, page.markdown.clone())
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_pdf_parsing() {
        dotenv().ok();
        let api_key = std::env::var("MISTRAL_API_KEY").expect("Missing MISTRAL_API_KEY");
        let document_url = "https://arxiv.org/pdf/2201.04234";

        let result = parse_pdf(document_url, &api_key).await.unwrap();
        println!("{}", result.len());
    }
}
