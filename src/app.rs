mod types;
use types::Prompt;
mod app {
    mod api;

    use super::Prompt;
    use api::*;
    use yew::prelude::*;

    #[function_component(App)]
    pub fn app() -> Html {
        let prompts = use_state(|| vec![Prompt::new()]);

        let onclick = {
            let prompts = prompts.clone();

            Callback::from(move |_| {
                // Use the `set` method to update the state
                let mut new_prompts = (*prompts).clone();
                new_prompts.push(Prompt {
                    reply: "Yes, you clicked that button",
                    message: "Yep, I clicked it",
                });
                prompts.set(new_prompts);
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
}
