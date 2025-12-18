//! LLM service adapter implementation

mod client;
mod stub;

pub use client::HttpLlmClient;
pub use stub::StubLlmClient;
