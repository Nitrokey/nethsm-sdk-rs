# Changelog

## Unreleased

### Breaking Changes

- `models`: Remove `KeyType::EcP224` and `TlsKeyType::EcP224` enum variants

### Features

- `models`: Add new enum variants:
  - `KeyMechanism`: `Bip340Signature`
  - `KeyType`: `EcP256K1`, `BrainpoolP256`, `BrainpoolP384`, `BrainpoolP512`
  - `SignMode`: `Bip340`
  - `TlsKeyType`: `BrainpoolP256`, `BrainpoolP384`, `BrainpoolP512`

## [v2.0.0](https://github.com/Nitrokey/nethsm-sdk-rs/releases/tag/v2.0.0) (2025-02-17)

- Update to  ureq 3.0.0 ([#35][])

[#35]: https://github.com/Nitrokey/nethsm-sdk-rs/pull/35

[All Changes](https://github.com/Nitrokey/nethsm-sdk-rs/compare/v1.1.1...HEAD)

## [v1.1.1](https://github.com/Nitrokey/nethsm-sdk-rs/releases/tag/v1.1.1) (2024-09-18)

### Features

- Implement `Display` for enums (@wiktor-k, [#33](https://github.com/Nitrokey/nethsm-sdk-rs/pull/33))

### Bugfixes

- Return correct error variants for API errors ([#30](https://github.com/Nitrokey/nethsm-sdk-rs/issues/30))

[All Changes](https://github.com/Nitrokey/nethsm-sdk-rs/compare/v1.1.0...v1.1.1)

## [v1.1.0](https://github.com/Nitrokey/nethsm-sdk-rs/releases/tag/v1.1.0) (2024-07-17)

### Features

- Add support for namespaces by adding the `namespaces_get`, `namespaces_namespace_id_delete`, `namespaces_namespace_id_put`, `users_user_id_post` API calls

### Bugfixes

- Return `Error::ResponseError` instead of `Error::Transport` for API errors ([#21](https://github.com/Nitrokey/nethsm-sdk-rs/issues/21))
- Fix multipart requests, namely `system_restore_post` ([#20](https://github.com/Nitrokey/nethsm-sdk-rs/issues/20))
- Add authentication for `system_restore_post` ([#15](https://github.com/Nitrokey/nethsm-sdk-rs/issues/15))

### Other Changes

- Add `AkPub` and `Pcr` schemas

[All Changes](https://github.com/Nitrokey/nethsm-sdk-rs/compare/v1.0.1...v1.1.0)

## [v1.0.1](https://github.com/Nitrokey/nethsm-sdk-rs/releases/tag/v1.0.1) (2024-05-06)

### Bugfixes

- Enable `alloc` feature for `base64` dependency ([#14](https://github.com/Nitrokey/nethsm-sdk-rs/issues/14))

[All Changes](https://github.com/Nitrokey/nethsm-sdk-rs/compare/v1.0.0...v1.0.1)

## [v1.0.0](https://github.com/Nitrokey/nethsm-sdk-rs/releases/tag/v1.0.0) (2023-11-27)

This is the first stable release of `nethsm-sdk-rs` and supports NetHSM [v1.0][nethsm-v1.0].

[nethsm-v1.0]: https://github.com/Nitrokey/nethsm/releases/tag/v1.0
