mod api;
//mod model;
mod types;
use web_sys::HtmlInputElement;
use yew::prelude::*;
#[function_component(App)]
pub fn app() -> Html {
    let prompts = use_state(|| vec![types::Prompt::new()]);
    let message = use_state(|| String::new());

    let oninput = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            message.set(input.value());
        })
    };

    let onclick = {
        print!("button clicked, waiting for gemma");
        let prompts = prompts.clone();
        let message = message.clone();
        Callback::from(move |_| {
            let prompts = prompts.clone();
            let message = message.clone();
            let request = types::Prompt {
                message: message.to_string(),
                reply: "".to_string(),
            };
            tokio::task::spawn_local(async move {
                let response = api::send(request, "codegemma:2b".to_string())
                    .await
                    .unwrap();
                let mut new_prompts = (*prompts).clone();
                new_prompts.push(response);
                prompts.set(new_prompts);
                message.set("".to_string());
            });
        })
    };

    html! {
        <div class="root">
                <div class="chat-section">
                {for prompts.iter().map(|prompt| html! {
                    <div>
                        <p>{ format!("You: {}", prompt.message) }</p>
                        <p>{ format!("Reply: {}", prompt.reply) }</p>
                    </div>
                })}
                </div>

                <div class="input-section">
                    <input class="input-prompt" placeholder="Start asking here..." />
                    <button class="send-btn" {onclick} > {"Go!"} </button>
                </div>
        </div>
    }
}
