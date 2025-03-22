use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    content: Content,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    embedding: Embedding,
}

#[derive(Debug, Deserialize)]
struct Embedding {
    values: Vec<f32>,
}

pub async fn embed(text: &str, api_key: &str) -> Result<Vec<f32>, Error> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-exp-03-07:embedContent?key={}",
        api_key
    );

    let request = EmbeddingRequest {
        model: "models/gemini-embedding-exp-03-07".to_string(),
        content: Content {
            parts: vec![Part {
                text: text.to_string(),
            }],
        },
    };

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await?
        .json::<EmbeddingResponse>()
        .await?;

    Ok(response.embedding.values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_text_embedding() {
        dotenv().ok();
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
        let text = "What is the meaning of life?";

        let embedding = embed(&text, &api_key).await.unwrap();
        assert_eq!(embedding.len(), 3072);
    }
}
