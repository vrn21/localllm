use yew::prelude::*;

struct Prompt<'a> {
    message: &'a str,
    reply: &'a str,
}

#[function_component]
fn App() -> Html {
    let prooompt = use_state(|| {
        vec![Prompt {
            message: "",
            reply: "Hello user! I hope you are having a good day, how can i help you :)",
        }];
    });

    let onclick = {
        let prooompt = prooompt.clone();

        Callback::from(move |_| {
            prooompt.push(Prompt {
                reply: "yes you clicked that button",
                message: "yep i clicked ig",
            })
        })
    };

    html! {
        <div>
            {"Welcome to locallm!"}
            <div class="chat-section">

                <div class="message-section">
                    {"our message to ai"}
                </div>

                <div class ="reply-section">
                    {"replies from ai"}
                </div>
                <div class="input-section">
                    <input class="input-prompt" placeholder="Start typing here..." />
                    <button class="send-btn" {onclick} > {"Go!"} </button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
