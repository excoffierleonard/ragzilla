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

pub async fn create_embedding(text: &str, api_key: &str) -> Result<Vec<f32>, reqwest::Error> {
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

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
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
    async fn test_embedding_creation() {
        dotenv().ok();
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

        let result = create_embedding("What is the meaning of life?", &api_key).await;
        assert!(result.is_ok());

        let embedding = result.unwrap();
        assert!(!embedding.is_empty());
    }
}
