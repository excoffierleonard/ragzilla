use reqwest::{
    Client, Error,
    multipart::{Form, Part},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct TranscriptionResponse {
    text: String,
}

pub async fn transcribe(audio_data: &[u8], api_key: &str) -> Result<String, Error> {
    let url = "https://api.openai.com/v1/audio/transcriptions";

    let form = Form::new().text("model", "gpt-4o-transcribe").part(
        "file",
        Part::bytes(audio_data.to_vec()).file_name("audio.mp3"),
    );

    let client = Client::new();
    let response = client
        .post(url)
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await?
        .json::<TranscriptionResponse>()
        .await?;

    Ok(response.text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[tokio::test]
    async fn test_audio_transcription() {
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
