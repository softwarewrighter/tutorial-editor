//! Avatar pipeline service adapter implementation

mod client;
mod http;
mod stub;

pub use client::HttpAvatarClient;
pub use stub::StubAvatarClient;
