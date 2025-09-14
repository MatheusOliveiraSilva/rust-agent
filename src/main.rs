use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

async fn ask_openai(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;

    let req = ChatRequest {
        model: "gpt-4.1", 
        messages: vec![
            Message { role: "system", content: "Você é um assistente útil." },
            Message { role: "user", content: prompt },
        ],
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&req)
        .send()
        .await?
        .error_for_status()?
        .json::<ChatResponse>()
        .await?;

    let answer = res
        .choices
        .into_iter()
        .next()
        .ok_or("sem choices na resposta")?
        .message
        .content;

    Ok(answer)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "Explique em uma frase o que é Rust.";
    let reply = ask_openai(input).await?;
    println!("Resposta: {}", reply);
    Ok(())
}
