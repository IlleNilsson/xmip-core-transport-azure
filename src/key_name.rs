//! The shared access policy a Location's Shared Access Signature is made
//! under, and the key the secret it names holds (ADR-0019, amendment
//! 2026-10-29; ADR-0015, amendment 2026-11-25): declared and held once for
//! azure-service-bus and azure-event-hubs. The key is a `password` secret,
//! kept in the Administration Database and read at each request
//! (`transport::secured::presented`), since a signature is made for every
//! request.

use transport::Secured;
use transport::error::Result;
use xcore::settings::{Applies, Kind, Presence, Setting};

/// The setting a Location names its shared access policy by.
pub const KEY_NAME: &str = "key_name";

/// The [`KEY_NAME`] setting as a technology declares it among its
/// `Configured::SETTINGS`, read on `applies`.
#[must_use]
pub const fn key_name(applies: Applies) -> Setting {
    Setting {
        name: KEY_NAME,
        kind: Kind::Text,
        presence: Presence::Optional,
        meaning: "The shared access policy a Shared Access Signature is made under, \
                  RootManageSharedAccessKey, with the key the Location's secret holds.",
        applies,
    }
}

/// `secured`, the secret a Location names, kept where `policy` is the
/// shared access policy its signature is made under: a Shared Access
/// Signature names its policy, so a Location naming a secret and no
/// `key_name` is refused as the node starts
/// (`transport::secured::without_holder`), as MQTT refuses a password with
/// no user name.
///
/// # Errors
/// Permanent, in words, where `policy` is empty.
pub fn signing_as(policy: &str, secured: Secured) -> Result<Secured> {
    if policy.is_empty() {
        return Err(transport::secured::without_holder(
            &secured,
            "the key of the shared access policy",
            KEY_NAME,
        ));
    }
    Ok(secured)
}
