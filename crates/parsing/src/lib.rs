use rayon::prelude::*;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    index: i32,
    markdown: String,
}

pub async fn parse_pdf(document_url: &str, api_key: &str) -> Result<HashMap<i32, String>, Error> {
    let url = "https://api.mistral.ai/v1/ocr";

    let request = ParsingRequest {
        model: "mistral-ocr-latest".to_string(),
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
        .into_par_iter()
        .map(|page| (page.index, page.markdown))
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

        let chunks = parse_pdf(document_url, &api_key).await.unwrap();
        println!("{}", chunks.len());
    }
}
