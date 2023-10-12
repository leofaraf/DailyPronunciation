use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccuracyRequest {
    #[serde(rename = "base64Audio")]
    pub base64audio: String,
    pub language: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccuracyResponse {
    pub real_transcript: String,
    pub is_letter_correct_all_words: String,
    pub pronunciation_accuracy: String,
}

