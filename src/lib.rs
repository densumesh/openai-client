//! # Getting started
//!
//! OpenAI Dive is an async Rust API client that allows you to interact with the OpenAI API.
//!
//! Sign up for an account on [https://platform.openai.com/overview](https://platform.openai.com/overview) to get your API token.
//!
//! ```ini
//! [dependencies]
//! openai_dive = "0.1"
//! ```
//!
//! ## Supported endpoints
//!
//! - Models
//!   - [List models](#list-models)
//!   - [Retrieve model](#retrieve-model)
//! - Completions
//!   - [Create completion](#create-completion)
//! - Chat
//!   - [Create chat completion](#create-chat-completion)
//! - Edit
//!   - [Create edit](#create-edit)
//!
//! ## Models
//!
//! You can hard-code any model name or use one of Enum options of `OpenAIModel`.
//!
//! - OpenAIModel::Chat3X5Turbo0301 = `gpt-3.5-turbo-0301`
//! - OpenAIModel::TextDavinciEdit001 = `text-davinci-edit-001`
//! - OpenAIModel::TextDavinci003 = `text-davinci-003`
//! - OpenAIModel::TextCurie001 = `text-curie-001`
//! - OpenAIModel::TextBabbage001 = `text-babbage-001`
//! - OpenAIModel::TextAda001 = `text-ada-001`
//!
//! # Examples
//!
//! ## List models
//!
//! Lists the currently available models, and provides basic information about each one such as the owner and availability.
//!
//! **URL** `https://api.openai.com/v1/models`
//!
//! **Method** `GET`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let api_key = "YOUR API KEY".to_string();
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.models().list().await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [List models](https://platform.openai.com/docs/api-reference/models/list)
//!
//! ## Retrieve model
//!
//! Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
//!
//! **URL** `https://api.openai.com/v1/models/{model}`
//!
//! **Method** `GET`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::models::OpenAIModel;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let api_key = "YOUR API KEY".to_string();
//!
//!     let model_id = OpenAIModel::TextDavinci003.to_string(); // text-davinci-003
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.models().get(model_id).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Retrieve models](https://platform.openai.com/docs/api-reference/models/retrieve)
//!
//! ## Create completion
//!
//! Creates a completion for the provided prompt and parameters.
//!
//! **URL** `https://api.openai.com/v1/completions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::completion::CompletionParameters;
//! use openai_dive::v1::models::OpenAIModel;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let api_key = "YOUR API KEY".to_string();
//!
//!     let parameters = CompletionParameters {
//!         model: OpenAIModel::TextDavinci003.to_string(), // text-davinci-003
//!         prompt: "Say this is a test".to_string(),
//!         max_tokens: 10,
//!         temperature: None,
//!         suffix: None,
//!     };
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.completions().create(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create completion](https://platform.openai.com/docs/api-reference/completions/create)
//!
//! ## Create chat completion
//!
//! Creates a completion for the chat message.
//!
//! **URL** `https://api.openai.com/v1/chat/completions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage};
//! use openai_dive::v1::models::OpenAIModel;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let api_key = "YOUR API KEY".to_string();
//!
//!     let parameters = ChatCompletionParameters {
//!         model: OpenAIModel::Chat3X5Turbo0301.to_string(), // gpt-3.5-turbo-0301
//!         messages: vec![
//!             ChatMessage {
//!                 role: "user".to_string(),
//!                 content: "Hello!".to_string(),
//!             },
//!         ],
//!         max_tokens: 12,
//!         temperature: None,
//!     };
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.chat().create(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)
//!
//! ## Create edit
//!
//! Creates a new edit for the provided input, instruction, and parameters.
//!
//! **URL** `https://api.openai.com/v1/edits`
//!
//! **Method** `POST`
//!
//! ```rust
//! use openai_dive::v1::api::Client;
//! use openai_dive::v1::resources::edit::EditParameters;
//! use openai_dive::v1::models::OpenAIModel;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let api_key = "YOUR API KEY".to_string();
//!
//!     let parameters = EditParameters {
//!         model: OpenAIModel::TextDavinciEdit001.to_string(), // text-davinci-edit-001
//!         input: "What day of the wek is it?".to_string(),
//!         instruction: "Fix the spelling mistakes".to_string(),
//!         temperature: None,
//!     };
//!
//!     let client = Client::new(api_key);
//!
//!     let result = client.edits().create(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create edit](https://platform.openai.com/docs/api-reference/edits/create)
//!

// OpenAI API v1
pub mod v1;
