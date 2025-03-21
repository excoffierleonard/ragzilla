# ragzilla-transcription

A Rust library for transcribing audio to text using OpenAI's GPT-4o transcription API.

## Features

- Convert audio data to text using OpenAI's transcription service
- Simple async API that returns transcribed text

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
ragzilla-transcription = "0.1.0"
```

## Usage

```rust
use ragzilla_transcription::transcribe;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let audio_data = fs::read("path/to/audio.mp3").expect("Could not read audio file");

    let transcription = transcribe(&audio_data, &api_key).await?;
    println!("Transcription: {}", transcription);

    Ok(())
}
```

## API Reference

### `transcribe`

```rust
pub async fn transcribe(audio_data: &[u8], api_key: &str) -> Result<String, reqwest::Error>
```

Transcribes the provided audio data to text using OpenAI's GPT-4o transcription service.

- `audio_data`: Raw audio data as a byte slice
- `api_key`: Your OpenAI API key

Returns a `Result` containing the transcribed text as a string.

## License

MIT License
