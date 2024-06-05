use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Prompt {
    pub message: String,
    pub reply: String,
}

#[derive(Serialize, Deserialize)]
struct Request {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct Response {
    response: String,
}

async fn send(prompt: Prompt, model: String) -> Result<Prompt, reqwest::Error> {
    let url = "http://localhost:11434/api/generate";
    let client: reqwest::Client = reqwest::Client::new();

    let request = Request {
        model: model,
        prompt: prompt.message.clone(),
        stream: false,
    };

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await?
        .json::<Response>()
        .await?;
    let mut prompt = prompt.clone();
    prompt.reply = response.response.as_str().to_owned();

    Ok(prompt.to_owned())
}

#[tokio::main]
async fn main() {
    let out = send(
        Prompt {
            message: "hey how are you?".to_string(),
            reply: "".to_string(),
        },
        "gemma:2b".to_string(),
    )
    .await
    .unwrap();
    println!("{}", out.reply);
}
