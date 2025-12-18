//! Low-level avatar service operations for OrchestratorApp

mod ops_impl;
mod ops_impl_tts;
mod ops_trait;
mod ops_trait_tts;

pub use ops_trait::AvatarServiceOps;
pub use ops_trait_tts::TtsOps;
