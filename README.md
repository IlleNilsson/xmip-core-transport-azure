# xmip-core-transport-azure

What every Azure technology speaks over HTTP: the Shared Access Signature and
Shared Key, both sides of each, and what a `servicebus.windows.net` namespace
answers when it refuses. Not a transport of its own:
[azure-blob](https://github.com/IlleNilsson/xmip-core-transport-azure-blob),
[azure-service-bus](https://github.com/IlleNilsson/xmip-core-transport-azure-service-bus)
and
[azure-event-hubs](https://github.com/IlleNilsson/xmip-core-transport-azure-event-hubs)
ride on it, and it rides on
[xmip-core-transport-http](https://github.com/IlleNilsson/xmip-core-transport-http).
A technology of
[xmip-core-transport](https://github.com/IlleNilsson/xmip-core-transport).

| module | what |
| --- | --- |
| `sas` | the Shared Access Signature, for azure-service-bus and azure-event-hubs |
| `key_name` | the `key_name` setting, the shared access policy a Location signs under, and its secret, for the same two |
| `shared_key` | Shared Key, for azure-blob |
| `namespace` | a namespace's refusal and its subcode, for the same two as `sas` |

Created 2026-09-24 on the owner's ruling of 2026-09-22: what one vendor speaks
leaves the http technology for a crate of that vendor's. The signature and the
namespace's answers lived in http from 2026-09-14, and Shared Key in
azure-blob (ADR-0044, amendment 2026-09-24).

A token's resource is read by `net::Target`: `Token::namespace` is the scheme and authority it lives under and `Token::path` the path it covers, which Service Bus and Event Hubs name their origins with; until 2026-09-28 each of the two cut it by hand, and so did the verifier. The token's fields are read by `codec::percent::decode_pairs`, and the RFC 1123 date by `net::http::date`, which the http transport held until the same day.

## The declaration

The setting azure-service-bus and azure-event-hubs declare, `key_name`, is a `const` here, `key_name::KEY_NAME` and `key_name::key_name` (ADR-0064, amendment 2026-11-26). Built with its `declaration` feature alone (`--no-default-features --features declaration`), the crate is that and nothing more: `key_name::signing_as`, both signatures, a namespace's refusal and every crate they are built on — the http technology, and with it TLS — are behind the default feature. A technology's own declaration takes it so, azure-event-hubs' and azure-service-bus' since 2026-11-26, and the library the surfaces load carries it through the gathering crate xmip-core-declaration (open problem 48). `test/Declaration.Test.ps1` in the estate holds that the declaration builds nothing the transport capability does not.

## Verification

The included workflow is manual-only and calls the versioned shared workflow at
`IlleNilsson/.github@v1`.
