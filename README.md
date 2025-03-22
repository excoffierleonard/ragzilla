# Ragzilla

A Rust library providing tools for RAG (Retrieval-Augmented Generation) pipelines.

## Features

Ragzilla provides several features that can be selectively enabled:

- `embedding`: Generate embeddings using Gemini API
- `parsing`: Parse PDFs using Mistral AI OCR API
- `transcribing`: Transcribe audio to text using OpenAI's GPT-4o API
- `all`: Enable all features

## Usage

Add ragzilla to your dependencies with the features you need:

```toml
[dependencies]
ragzilla = { version = "0.3.0", features = ["embedding", "parsing"] }
```

### Embedding Example

```rust
use ragzilla::embedding;

async fn create_embedding() {
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let text = "What is the meaning of life?";
    
    let embedding = embedding::embed(text, &api_key).await.unwrap();
    println!("Generated embedding with {} dimensions", embedding.len());
}
```

### PDF Parsing Example

```rust
use ragzilla::parsing;

async fn parse_document() {
    let api_key = std::env::var("MISTRAL_API_KEY").expect("MISTRAL_API_KEY must be set");
    let document_url = "https://example.com/document.pdf";
    
    let chunks = parsing::parse_pdf(document_url, &api_key).await.unwrap();
    println!("Extracted {} pages from PDF", chunks.len());
}
```

### Audio Transcription Example

```rust
use ragzilla::transcribing;
use std::fs;

async fn transcribe_audio() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let audio_data = fs::read("path/to/audio.mp3").expect("Could not read audio file");
    
    let text = transcribing::transcribe(&audio_data, &api_key).await.unwrap();
    println!("Transcription: {}", text);
}
```

## License

MIT
