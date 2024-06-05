pub mod types {
    #[derive(Clone)]
    pub struct Prompt<'a> {
        pub message: &'a str,
        pub reply: &'a str,
    }

    impl<'a> Prompt<'a> {
        pub fn new() -> Self {
            Prompt {
                message: "gm",
                reply: "Hello user! I hope you are having a good day, how can I help you :)",
            }
        }
    }
}
