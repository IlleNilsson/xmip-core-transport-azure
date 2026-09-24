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
| `shared_key` | Shared Key, for azure-blob |
| `namespace` | a namespace's refusal and its subcode, for the same two as `sas` |

Created 2026-09-24 on the owner's ruling of 2026-09-22: what one vendor speaks
leaves the http technology for a crate of that vendor's. The signature and the
namespace's answers lived in http from 2026-09-14, and Shared Key in
azure-blob (ADR-0044, amendment 2026-09-24).

## Toolchain

`rust-toolchain.toml` pins the toolchain for the whole estate. Do not change it
here.

## Verification

The included workflow is manual-only and calls the versioned shared workflow at
`IlleNilsson/.github@v1`.
