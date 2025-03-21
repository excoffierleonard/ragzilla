# ragzilla-parsing

A Rust library for parsing PDFs using the Mistral AI OCR API.

## Features

- Convert PDFs to markdown text using Mistral AI's OCR capabilities
- Simple async API that returns parsed content as a collection of text chunks

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
ragzilla-parsing = "0.1.0"
```

## Usage

```rust
use ragzilla_parsing::parse_pdf;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("MISTRAL_API_KEY").expect("MISTRAL_API_KEY must be set");
    let document_url = "https://arxiv.org/pdf/2201.04234";

    let chunks = parse_pdf(document_url, &api_key).await?;
    println!("Parsed {} pages from PDF", chunks.len());

    Ok(())
}
```

## API Reference

### `parse_pdf`

```rust
pub async fn parse_pdf(document_url: &str, api_key: &str) -> Result<Vec<String>, reqwest::Error>
```

Parses a PDF document from a URL and converts it to markdown text using Mistral AI's OCR service.

- `document_url`: URL to the PDF document to be parsed
- `api_key`: Your Mistral AI API key

Returns a `Result` containing a vector of strings, where each string represents the markdown content of a page.

## License

MIT License
