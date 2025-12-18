//! Avatar pipeline service adapter implementation

mod client;
mod http;
mod stub;

use orchestrator_domain::AvatarServices;
use reqwest::Client;

pub use stub::StubAvatarClient;

/// HTTP-based avatar pipeline client
pub struct HttpAvatarClient {
    pub(crate) client: Client,
    pub(crate) config: AvatarServices,
}

impl HttpAvatarClient {
    pub fn new(config: AvatarServices) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}
