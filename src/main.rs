use yew::prelude::*;

#[derive(Clone)]
struct Prompt<'a> {
    message: &'a str,
    reply: &'a str,
}

#[function_component(App)]
fn app() -> Html {
    // Correctly initialize the state as a vector of Prompt structs
    let prompts = use_state(|| {
        vec![Prompt {
            message: "",
            reply: "Hello user! I hope you are having a good day, how can I help you :)",
        }]
    });

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
        <div>
            {"Welcome to locallm!"}
                <div class="chat-section">
                {for prompts.iter().map(|prompt| html! {
                    <div>
                        <p>{ format!("Message: {}", prompt.message) }</p>
                        <p>{ format!("Reply: {}", prompt.reply) }</p>
                    </div>
                })}
                </div>

                <div class="input-section">
                    <input class="input-prompt" placeholder="Start typing here..." />
                    <button class="send-btn" {onclick} > {"Go!"} </button>
                </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

