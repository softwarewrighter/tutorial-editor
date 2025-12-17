use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub services: ServicesConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub sqlite_path: String,
    pub assets_root: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServicesConfig {
    pub llm: LlmServices,
    pub avatar: AvatarServices,
    pub tts: TtsServices,
    pub mcp: McpServices,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LlmServices {
    pub cloud_primary: String,
    pub local_ollama: String,
    pub local_llamacpp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AvatarServices {
    pub image_to_video: String,
    pub lip_sync: String,
    pub background_removal: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TtsServices {
    pub primary: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct McpServices {
    pub playwright_hub: String,
}
