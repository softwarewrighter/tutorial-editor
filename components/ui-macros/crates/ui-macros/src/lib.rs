//! UI macros for reducing boilerplate in Yew applications
//!
//! This crate provides declarative macros for common UI patterns:
//! - Callback builders (show_callback!, hide_callback!, etc.)
//! - Form field generators (text_field!, textarea_field!, etc.)
//!
//! Macros are automatically exported at crate root via #[macro_export].

mod callback;
mod form;
