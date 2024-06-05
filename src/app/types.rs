#[derive(Clone)]
pub struct Prompt {
    pub message: String,
    pub reply: String,
}

impl Prompt {
    pub fn new() -> Self {
        Prompt {
            message: "gm".to_string(),
            reply: "Hello user! I hope you are having a good day, how can I help you :)"
                .to_string(),
        }
    }
}
