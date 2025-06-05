# crates_llms_txt JS/TS Library

[![npm version][npm-version-src]][npm-version-href]
[![npm downloads][npm-downloads-src]][npm-downloads-href]
[![License][license-src]][license-href]

This library provides a standard interface to fetch and parse Rust crate documentation and session data for use with LLMs (Large Language Models).

## Features

- Fetches documentation and session data for any Rust crate by name and version
- Returns a unified configuration object for LLM consumption
- TypeScript type definitions included

## Installation

```bash
npm install crates-llms-txt-napi
```

## Usage Example

```typescript
import { get_llms_standard_config } from 'crates-llms-txt-napi'

const config = await get_llms_standard_config('clap', '4.5.39')
```

## API

### get_llms_standard_config(lib_name: string, version?: string): Promise<LLMsStandardConfig>

Fetches the standard configuration for a given Rust crate and version.

- lib_name : The name of the crate (e.g., "clap")
- version : The version string (optional, defaults to latest)
- Returns: LLMsStandardConfig object LLMsStandardConfig Type

```ts
type SessionItem = { title: string; description: string; link: string }
type FullSessionItem = { content: string; link: string }

export type LLMsStandardConfig = {
  libName: string
  version: string
  sessions: SessionItem[]
  fullSessions: FullSessionItem[]
}
```

<!-- Badges -->

[npm-version-src]: https://img.shields.io/npm/v/crates-llms-txt-napi?style=flat&colorA=080f12&colorB=1fa669
[npm-version-href]: https://npmjs.com/package/crates-llms-txt-napi
[npm-downloads-src]: https://img.shields.io/npm/dm/crates-llms-txt-napi?style=flat&colorA=080f12&colorB=1fa669
[npm-downloads-href]: https://npmjs.com/package/crates-llms-txt-napi
[license-src]: https://img.shields.io/github/license/kingsword09/crates_llms_txt.svg?style=flat&colorA=080f12&colorB=1fa669
[license-href]: https://github.com/kingsword09/crates_llms_txt/blob/main/LICENSE
