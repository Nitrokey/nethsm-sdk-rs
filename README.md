# Rust API client for NetHSM

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.

- API version: v1
- Package version: 0.1.0
- Build package: `com.nitrokey.crustgen.CrustGenerator`

All endpoints expect exactly the specified JSON.
Additional properties will cause a Bad Request Error (400).
All HTTP errors contain a JSON structure with an explanation of type string.
All [base64](https://tools.ietf.org/html/rfc4648#section-4) encoded values are Big Endian.

## Installation

Put the package under your project folder in a directory named `nethsm-sdk` and add the following to `Cargo.toml` under `[dependencies]`:

```toml
nethsm-sdk-rs = { git = "https://github.com/Nitrokey/nethsm-sdk-rs" }
```

To get access to the crate's generated documentation, use:

```sh
cargo doc --open
```

## Updating the code

Dependencies : Rust toolchain, Docker.

To update the code from the latest api definition, run :

```sh
./update_openapi.sh
```

It will build the custom openapi-generator docker image, run it to generate the code from the online api definition and then format the generated code.

## Licence

The Rust crate is licensed under the MIT license.  
The customized Openapi generator code is modified from the openapi-generator repository, licenced under Apache 2.0.
