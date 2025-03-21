# ragzilla-embedding

A Rust library for creating text embeddings using the Gemini API.

## Features

- Generate text embeddings using Google's Gemini API
- Simple async API that returns embedding vectors

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
ragzilla-embedding = "0.1.0"
```

## Usage

```rust
use ragzilla_embedding::create_embedding;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let text = "What is the meaning of life?";

    let embedding = create_embedding(text, &api_key).await?;
    println!("Generated embedding with {} dimensions", embedding.len());

    Ok(())
}
```

## API Reference

### `create_embedding`

```rust
pub async fn create_embedding(text: &str, api_key: &str) -> Result<Vec<f32>, reqwest::Error>
```

Generates an embedding vector for the provided text using the Gemini API.

- `text`: The text to create an embedding for
- `api_key`: Your Gemini API key

Returns a `Result` containing a vector of floating-point values representing the embedding.

## License

MIT License
