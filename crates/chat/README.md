# openai-client-base-chat

An internal part of [`openai-client-base`](https://crates.io/crates/openai-client-base). Depend on `openai-client-base`
instead: it re-exports this crate under its own paths (`openai_client_base::models`, `openai_client_base::apis`),
behind the same Cargo features.

This crate holds the models and API functions of the features `chat-completion`, `completions`, `embedding`, `image`, `model`, `moderation`.

`openai-client-base` is generated as several crates so that each one compiles in less memory. Which crate holds a model
or an API module follows from the API specification and changes with it: the paths of this crate are not a stable
API. Its version is always that of `openai-client-base`, which requires exactly that version.
