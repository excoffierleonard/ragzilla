use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct WhisperResponse {
    text: String,
}

pub async fn transcribe(audio_data: &[u8], api_key: &str) -> Result<String, Error> {
    let url = "https://api.openai.com/v1/audio/transcriptions";
    let client = Client::new();

    let form = reqwest::multipart::Form::new()
        .text("model", "whisper-1")
        .part(
            "file",
            reqwest::multipart::Part::bytes(audio_data.to_vec())
                .file_name("audio.mp3")
                .mime_str("audio/mpeg")
                .unwrap(),
        );

    let response = client
        .post(url)
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await?
        .json::<WhisperResponse>()
        .await?;

    Ok(response.text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[tokio::test]
    async fn test_whisper_transcription() {
        dotenvy::dotenv().ok();
        let api_key = std::env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
        let audio_data = fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("assets")
                .join("test_audio.mp3"),
        )
        .expect("Could not read audio file");

        let transcription = transcribe(&audio_data, &api_key).await.unwrap();
        assert_eq!(transcription, "Hello, how are you?");
    }
}
