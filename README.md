# Localllm

Localllm is a web-based chat application built entirely with Rust, demonstrating the potential of frontend web development using Yew. This project showcases a locally running LLM (Language Model) with a seamless chat interface.


## Table of Contents
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Features

- 🌟 **100% Rust**: Entirely built with Rust, showcasing the power and versatility of the language.
- 🖥️ **Frontend with Yew**: Utilizes Yew for building the frontend, demonstrating Rust's capability in web development.
- 🤖 **Local LLM Integration**: Chat with a locally running language model.

## Getting Started

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Yew](https://yew.rs/docs/getting-started/installation)

### Installation

Clone the repository:

git clone https://github.com/yourusername/localllm.git
cd localllm
cargo run

Usage
Open your browser and navigate to http://localhost:8080.
Start chatting with the locally running LLM model.

<h3>Project Structure</h3>
<!-- localllm
├── src 
│   ├── app.rs    \  // Frontend and API calling 
│   ├── api.rs     \   // API logic
│   ├── types.rs    \  // Struct definitions
│   └── main.rs      \ // Entry point, calling the app function
├── Cargo.toml       \ // Cargo configuration file
└── README.md         \  // Project README file -->
<h4>app.rs</h4>
Handles the complete frontend and API calling logic.

<h4>api.rs</h4>
Contains the logic behind the API interactions.

<h4>types.rs</h4>
Defines the necessary structs used across the application.

<h4>main.rs</h4>
The entry point of the application, where the app function is called.

<h3>Contributing</h3>
Contributions are welcome! Please fork this repository and submit pull requests.

Fork the repository.
Create your feature branch (git checkout -b feature/AmazingFeature).
Commit your changes (git commit -m 'Add some AmazingFeature').
Push to the branch (git push origin feature/AmazingFeature).
Open a pull request.

<h1 style={display:flex; justify-content:center; align-items:center}>Made with ❤️ using Rust and Yew</h1>
