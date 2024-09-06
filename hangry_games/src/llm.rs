// main.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct RequestBody {
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct ResponseBody {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

pub fn send_text_to_llm(text: &str) -> Result<()> {
    // Create an HTTP client
    let client = Client::new();

    // Define the request body
    let request_body = RequestBody {
        prompt: "Translate the following English text to French: 'Hello, world!'".to_string(),
        max_tokens: 60,
    };

    // Send the request
    let response = client
        .post("http://localhost:1234/v1/chat/completions")
        .json(&request_body)
        .send()
        .await?;

    // Parse the response
    let response_body: ResponseBody = response.json().await?;

    // Print the response
    if let Some(choice) = response_body.choices.first() {
        println!("Response: {}", choice.text);
    }

    Ok(())
}