# crates_llms_txt

[![Crates.io][crates-src]][crates-href]
[![crates downloads][crates-download-src]][crates-download-href]
[![napi version][npm-napi-version-src]][npm-napi-version-href]
[![napi downloads][npm-napi-downloads-src]][npm-napi-downloads-href]
[![npm version][npm-version-src]][npm-version-href]
[![npm downloads][npm-downloads-src]][npm-downloads-href]
[![License][license-src]][license-href]

A repository for generating content for `llms.txt` and `llms-full.txt` files used by Rust libraries.

## Package Distribution

This repository provides three main distribution formats/packages:

### Rust Library: `crates_llms_txt`

The `crates_llms_txt` library is the native Rust implementation.

- **Crates.io:** [crates_llms_txt](https://crates.io/crates/crates_llms_txt)
- **Source:** `rs-lib/`

**Description:** An asynchronous Rust library for retrieving Rust crates documentation structure and LLM configurations.

**Features:**

- Asynchronously fetch crates documentation index.
- Parse documentation structure into Rust structs.
- Generate standard LLM configurations, including sessions and full_sessions.

**Usage:**

To use `crates_llms_txt` in your Rust project, add it as a dependency to your `Cargo.toml`:

```toml
[dependencies]
crates_llms_txt = "0.0.8" # Replace with the latest version
```

Then, you can use its functions in your Rust code. For example, to fetch documentation (conceptual):

```rust
use crates_llms_txt::get_llms_config_online;

async fn main() {
    match get_llms_config_online("clap", Some("4.5.39".to_string())).await {
        Ok(data) => {
            // Process data
            println!("Successfully fetched data.");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

#### API

This section details the methods available on the `LLMsStandardConfig` struct. To use these, you would typically first obtain or create an `LLMsStandardConfig` instance if needed, or call static/async methods directly if applicable. The primary public interface also includes top-level functions like `get_llms_config_online` (shown in the example above) which often wrap these methods.

##### `LLMsStandardConfig::new(lib_name: &str, version: &str) -> Self`

Creates a new `LLMsStandardConfig` instance. This function is used internally to initialize a configuration object.

- `lib_name: &str`: The name of the crate.
- `version: &str`: The version of the crate.
- **Returns:** `Self` - A new `LLMsStandardConfig` instance.

##### `async LLMsStandardConfig::get_llms_config_online(lib_name: &str, version: Option<String>) -> Result<LLMsStandardStringConfig, Box<dyn Error>>`

Get the LLM config for a given crate and version by fetching documentation from online sources (docs.rs).

- `lib_name: &str`: The name of the crate.
- `version: Option<String>`: The version of the crate. If `None`, the latest version will be used.
- **Returns:** `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The LLM config for the crate, with sessions and full_sessions serialized as JSON strings.

##### `async LLMsStandardConfig::get_llms_config_online_by_url(url: &str) -> Result<LLMsStandardStringConfig, Box<dyn Error>>`

Get the LLM config for a given crate by providing a direct URL to the `cratename/latest/json` documentation endpoint.

- `url: &str`: The direct URL to the crate's JSON documentation index (e.g., "https://docs.rs/crate/clap/latest/json").
- **Returns:** `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The LLM config for the crate, with sessions and full_sessions serialized as JSON strings.

##### `LLMsStandardConfig::get_llms_config_offline_with_all_features(toolchain: &str, manifest_path: PathBuf) -> Result<LLMsStandardStringConfig, Box<dyn Error>>`

Generate documentation for a crate using offline mode with all features enabled. This function invokes `cargo doc` locally.

- `toolchain: &str`: The Rust toolchain to use (e.g., "stable", "nightly").
- `manifest_path: PathBuf`: Path to the `Cargo.toml` file of the crate.
- **Returns:** `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The generated documentation config, with sessions and full_sessions serialized as JSON strings.

_Note: This function is only available when the `rustdoc` feature is enabled._

##### `LLMsStandardConfig::get_llms_config_offline_with_features(toolchain: &str, manifest_path: PathBuf, no_default_features: bool, features: Option<Vec<String>>) -> Result<LLMsStandardStringConfig, Box<dyn Error>>`

Generate documentation for a crate using offline mode with specified features enabled. This function invokes `cargo doc` locally.

- `toolchain: &str`: The Rust toolchain to use (e.g., "stable", "nightly").
- `manifest_path: PathBuf`: Path to the `Cargo.toml` file of the crate.
- `no_default_features: bool`: Whether to disable the default features.
- `features: Option<Vec<String>>`: List of features to enable.
- **Returns:** `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The generated documentation config, with sessions and full_sessions serialized as JSON strings.

_Note: This function is only available when the `rustdoc` feature is enabled._

For detailed API usage, including the top-level helper functions, please refer to the documentation within the `rs-lib` directory or on [crates.io](https://crates.io/crates/crates_llms_txt).

### NAPI Package: `crates-llms-txt-napi`

The `crates-llms-txt-napi` library is the JS/TS implementation, distributed via npm.

- **NPM Package:** [crates-llms-txt-napi](https://www.npmjs.com/package/crates-llms-txt-napi)
- **Source:** `napi/`

**Description:** This library provides a standard interface to fetch and parse Rust crate documentation and session data for use with LLMs (Large Language Models).

**Features:**

- Fetches documentation and session data for any Rust crate by name and version.
- Returns a unified configuration object for LLM consumption.
- TypeScript type definitions included.

**Installation:**

```bash
npm install crates-llms-txt-napi
```

**Usage Example:**

```typescript
import { getLlmsConfigOnlineByCratesName } from "crates-llms-txt-napi";

async function main() {
  const config = await getLlmsConfigOnlineByCratesName("clap", "4.5.39");
}

main();
```

**API:**

#### `async getLlmsConfigOnlineByCratesName(libName: string, version?: string): Promise<LlMsConfig | null>`

Fetches the standard configuration for a given Rust crate and version from online sources (docs.rs).

- `libName: string`: The name of the crate (e.g., "clap").
- `version?: string`: The version string (optional, if not provided or `undefined`, the latest version of the crate will be attempted).
- **Returns:** `Promise<LlMsConfig | null>` - A promise that resolves to the `LlMsConfig` object or `null` if an error occurs (e.g., crate not found, network issue).

#### `async getLlmsConfigOnlineByUrl(url: string): Promise<LlMsConfig | null>`

Fetches the standard configuration for a Rust crate by providing a direct URL to its `docs.rs` JSON documentation file.

- `url: string`: The direct URL to the crate's JSON documentation index (e.g., "https://docs.rs/crate/clap/latest/json").
- **Returns:** `Promise<LlMsConfig | null>` - A promise that resolves to the `LlMsConfig` object or `null` if an error occurs (e.g., URL not reachable, invalid JSON).

#### `getLlmsConfigByRustdocAllFeatures(toolchain: string, manifestPath: string): LlMsConfig | null`

Generates the LLM configuration for a local Rust crate by invoking `cargo doc` with all features enabled. This function requires a local Rust toolchain.

- `toolchain: string`: The Rust toolchain to use (e.g., "stable", "nightly").
- `manifestPath: string`: Absolute or relative path to the `Cargo.toml` file of the local crate.
- **Returns:** `LlMsConfig | null` - The `LlMsConfig` object or `null` if an error occurs during documentation generation.

#### `getLlmsConfigByRustdocFeatures(toolchain: string, manifestPath: string, noDefaultFeatures: boolean, features?: string[]): LlMsConfig | null`

Generates the LLM configuration for a local Rust crate by invoking `cargo doc` with specified features. This function requires a local Rust toolchain.

- `toolchain: string`: The Rust toolchain to use (e.g., "stable", "nightly").
- `manifestPath: string`: Absolute or relative path to the `Cargo.toml` file of the local crate.
- `noDefaultFeatures: boolean`: If `true`, disables the default features of the crate.
- `features?: string[]`: An optional list of features to enable.
- **Returns:** `LlMsConfig | null` - The `LlMsConfig` object or `null` if an error occurs during documentation generation.

**`LlMsConfig` Type:**

```typescript
type SessionItem = { title: string; description: string; link: string };
type FullSessionItem = { content: string; link: string };

export type LlMsConfig = {
  libName: string;
  version: string;
  sessions: string /*SessionItem[]*/;
  fullSessions: string /*FullSessionItem[]*/;
};
```

#### Supported Architectures

The `crates-llms-txt-napi` package provides prebuilt binaries for the following target architectures:

| Target Triple                   |
| ------------------------------- |
| `x86_64-apple-darwin`           |
| `aarch64-apple-darwin`          |
| `x86_64-pc-windows-msvc`        |
| `aarch64-pc-windows-msvc`       |
| `i686-pc-windows-msvc`          |
| `x86_64-unknown-linux-gnu`      |
| `x86_64-unknown-linux-musl`     |
| `aarch64-unknown-linux-gnu`     |
| `aarch64-unknown-linux-musl`    |
| `armv7-unknown-linux-gnueabihf` |

### NPM Package: `crates-llms-txt`

[![npm version][npm-main-version-src]][npm-main-version-href] [![npm downloads][npm-main-downloads-src]][npm-main-downloads-href]

This is a higher-level JavaScript/TypeScript wrapper over the `crates-llms-txt-napi` package. It provides a more user-friendly interface by automatically parsing the JSON strings for `sessions` and `fullSessions` into structured JavaScript objects.

- **NPM Package:** [crates-llms-txt](https://www.npmjs.com/package/crates-llms-txt)
- **Source:** `npm/`

**Installation:**

```bash
npm install crates-llms-txt
# or
yarn add crates-llms-txt
# or
pnpm add crates-llms-txt
```

**Usage Example:**

```typescript
import { getLlmsConfigOnlineByCratesName } from "crates-llms-txt";

async function main() {
  const config = await getLlmsConfigOnlineByCratesName("tokio", "1"); // Example: Get latest 1.x for tokio
  if (config) {
    console.log(`Fetched config for ${config.libName} v${config.version}`);
    config.sessions.forEach(session => {
      console.log(`  Session: ${session.title} - ${session.link}`);
    });
    // config.fullSessions.forEach(session => {
    //   console.log(`  Full Session Content (first 50 chars): ${session.content.substring(0, 50)}...`);
    // });
  } else {
    console.log("Failed to fetch LLMs config.");
  }
}

main();
```

#### API

All functions return `null` if the underlying NAPI call fails or if the JSON parsing of session data is unsuccessful.

##### `async getLlmsConfigOnlineByCratesName(libName: string, version?: string): Promise<LLMsStandardConfig | null>`

Get the LLMs config from the online API by crates name. `sessions` and `fullSessions` are parsed into objects.

-   `libName: string`: The name of the library.
-   `version?: string`: The version of the library. (Optional, defaults to latest if not specified).
-   **Returns:** `Promise<LLMsStandardConfig | null>` - The LLMs config with parsed sessions, or `null` on error.

##### `async getLlmsConfigOnlineByUrl(url: string): Promise<LLMsStandardConfig | null>`

Get the LLMs config from the online API by URL. `sessions` and `fullSessions` are parsed into objects.

-   `url: string`: The direct URL to the crate's JSON documentation index.
-   **Returns:** `Promise<LLMsStandardConfig | null>` - The LLMs config with parsed sessions, or `null` on error.

##### `getLlmsConfigByRustdocAllFeatures(toolchain: string, manifestPath: string): LLMsStandardConfig | null`

Get the LLMs config by generating documentation locally using `rustdoc` with all features enabled. `sessions` and `fullSessions` are parsed into objects. Requires a local Rust toolchain.

-   `toolchain: string`: The Rust toolchain to use (e.g., "stable", "nightly").
-   `manifestPath: string`: Path to the `Cargo.toml` file of the crate.
-   **Returns:** `LLMsStandardConfig | null` - The LLMs config with parsed sessions, or `null` on error.

##### `getLlmsConfigByRustdocFeatures(toolchain: string, manifestPath: string, noDefaultFeatures: boolean, features?: string[]): LLMsStandardConfig | null`

Get the LLMs config by generating documentation locally using `rustdoc` with specified features. `sessions` and `fullSessions` are parsed into objects. Requires a local Rust toolchain.

-   `toolchain: string`: The Rust toolchain to use (e.g., "stable", "nightly").
-   `manifestPath: string`: Path to the `Cargo.toml` file of the crate.
-   `noDefaultFeatures: boolean`: Whether to disable the default features.
-   `features?: string[]`: Optional list of features to enable.
-   **Returns:** `LLMsStandardConfig | null` - The LLMs config with parsed sessions, or `null` on error.

#### Types

These are the primary types returned by the API functions.

```typescript
/**
 * SessionItem is the session item.
 */
export type SessionItem = {
  title: string;
  description: string;
  link: string;
};

/**
 * FullSessionItem is the full session item.
 */
export type FullSessionItem = {
  content: string;
  link: string;
};

/**
 * LLMsStandardConfig is the standard config for LLMs.
 * It contains the library name, version, and parsed session data.
 */
export type LLMsStandardConfig = {
  libName: string;
  version: string;
  sessions: SessionItem[];
  fullSessions: FullSessionItem[];
};
```

## License

MIT License

<!-- Badges -->

[npm-napi-version-src]: https://img.shields.io/npm/v/crates-llms-txt-napi?style=flat&colorA=080f12&colorB=1fa669&label=napi
[npm-napi-version-href]: https://npmjs.com/package/crates-llms-txt-napi
[npm-napi-downloads-src]: https://img.shields.io/npm/dm/crates-llms-txt-napi?style=flat&colorA=080f12&colorB=1fa669&label=napi-download
[npm-napi-downloads-href]: https://npmjs.com/package/crates-llms-txt-napi
[npm-version-src]: https://img.shields.io/npm/v/crates-llms-txt?style=flat&colorA=080f12&colorB=1fa669
[npm-version-href]: https://npmjs.com/package/crates-llms-txt
[npm-downloads-src]: https://img.shields.io/npm/dm/crates-llms-txt?style=flat&colorA=080f12&colorB=1fa669&label=npm-download
[npm-downloads-href]: https://npmjs.com/package/crates-llms-txt
[license-src]: https://img.shields.io/github/license/kingsword09/crates_llms_txt.svg?style=flat&colorA=080f12&colorB=1fa669
[license-href]: https://github.com/kingsword09/crates_llms_txt/blob/main/LICENSE
[crates-src]: https://img.shields.io/crates/v/crates_llms_txt
[crates-href]: https://crates.io/crates/crates_llms_txt
[crates-download-src]: https://img.shields.io/crates/d/crates_llms_txt?label=crates-download
[crates-download-href]: https://crates.io/crates/crates_llms_txt
