//! TTS service adapter implementation

mod client;
mod stub;

pub use client::HttpTtsClient;
pub use stub::StubTtsClient;
