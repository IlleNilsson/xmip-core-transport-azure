#![forbid(unsafe_code)]

//! What every Azure technology speaks over HTTP. Not a transport of its
//! own: azure-blob, azure-service-bus and azure-event-hubs ride on it, and
//! on the http technology beneath it.
//!
//! ```text
//! sas.rs         the Shared Access Signature, both sides, for
//!                azure-service-bus and azure-event-hubs
//! shared_key.rs  Shared Key, both sides, for azure-blob
//! namespace.rs   what a servicebus.windows.net namespace answers when it
//!                refuses, for azure-service-bus and azure-event-hubs
//! ```
//!
//! The signature and the namespace's answers lived in the http technology
//! from 2026-09-14, when azure-event-hubs was found importing
//! azure-service-bus, and Shared Key in azure-blob. The owner ruled on
//! 2026-09-22 that what one vendor speaks leaves HTTP for a crate of that
//! vendor's: http keeps HTTP, and a technology riding on Azure depends on
//! this crate, never on a sibling (ADR-0044).

pub mod namespace;
pub mod sas;
pub mod shared_key;
