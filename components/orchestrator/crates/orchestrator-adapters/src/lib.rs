mod avatar_adapter;
mod llm_adapter;
mod mcp_adapter;
mod stubs;
mod tts_adapter;

pub use avatar_adapter::HttpAvatarClient;
pub use llm_adapter::HttpLlmClient;
pub use mcp_adapter::HttpMcpClient;
pub use stubs::{StubAvatarClient, StubLlmClient, StubMcpClient, StubTtsClient};
pub use tts_adapter::HttpTtsClient;
