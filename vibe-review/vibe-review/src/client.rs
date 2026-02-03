use std::env;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-sonnet-4-20250514";
const MAX_TOKENS: u32 = 1000;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("missing ANTHROPIC_API_KEY environment variable")]
    MissingApiKey,
    #[error("request failed: {0}")]
    Request(String),
    #[error("response parse failed: {0}")]
    Response(String),
}

#[derive(Debug, Serialize)]
struct MessageRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    system: &'a str,
    messages: Vec<UserMessage<'a>>,
}

#[derive(Debug, Serialize)]
struct UserMessage<'a> {
    role: &'a str,
    content: Vec<MessageContent<'a>>,
}

#[derive(Debug, Serialize)]
struct MessageContent<'a> {
    #[serde(rename = "type")]
    content_type: &'a str,
    text: &'a str,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: Vec<ResponseContent>,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

pub fn send_review_request(prompt: &str) -> Result<String, ClientError> {
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| ClientError::MissingApiKey)?;
    let client = Client::new();

    let request = MessageRequest {
        model: MODEL,
        max_tokens: MAX_TOKENS,
        system: "You are a helpful Rust code review assistant.",
        messages: vec![UserMessage {
            role: "user",
            content: vec![MessageContent {
                content_type: "text",
                text: prompt,
            }],
        }],
    };

    let response = client
        .post(API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request)
        .send()
        .map_err(|err| ClientError::Request(err.to_string()))?;

    let response = response
        .error_for_status()
        .map_err(|err| ClientError::Request(err.to_string()))?;

    let message: MessageResponse = response
        .json()
        .map_err(|err| ClientError::Response(err.to_string()))?;

    let mut combined = String::new();
    for item in message.content {
        if item.content_type == "text"
            && let Some(text) = item.text
        {
            combined.push_str(&text);
        }
    }

    if combined.is_empty() {
        return Err(ClientError::Response(String::from("empty response")));
    }

    Ok(combined)
}
