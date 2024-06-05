use serde::{Serialize, Deserialize};

#[derive(Clone)]
    pub struct Prompt<'a> {
        pub message: &'a str,
        pub reply: &'a str,
    }

#[derive(Serialize, Deserialize)]
struct POST{
    model: &str,
    prompt: &str,
}

#[derive(Serialize, Deserialize)]
struct Request<'a>{
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct Response{
    response :String,
}

#[tokio::main]
async fn send(prompt: Prompt<'a>,model: &str) -> Result<Prompt<'a>',reqwest::Error>{
    let url = "http://localhost:11434/api/generate";
    let client: reqwest::Client = reqwest::Client::new();

    let request = Request{
        model: "guru",
        prompt: prompt.message.to_string(),
        stream: false,
    };

    let response = client.post(url).json(&request).send().await()?.json::<Response>().await?;
    let prompt = prompt.clone();
    prompt.reply = reponse.response;

    Ok(prompt)
}

fn main(){
    let out: Prompt = send(Prompt{
        message:"hey how are you?",
        reply:"",
    },"gemma:2b").unwrap;
}
