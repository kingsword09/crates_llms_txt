# crates_llms_txt JS/TS Library

This library provides a standard interface to fetch and parse Rust crate documentation and session data for use with LLMs (Large Language Models).

## Features

- Fetches documentation and session data for any Rust crate by name and version
- Returns a unified configuration object for LLM consumption
- TypeScript type definitions included

## Installation

```bash
deno install jsr:@kingsword09/crates-llms-txt
# or
npm install crates-llms-txt
```

## Usage Example

```typescript
import { get_llms_standard_config } from "jsr:@kingsword09/crates-llms-txt";

const config = await get_llms_standard_config("clap", "4.5.39");
```

## API

### get_llms_standard_config(lib_name: string, version?: string): Promise<LLMsStandardConfig>

Fetches the standard configuration for a given Rust crate and version.

- lib_name : The name of the crate (e.g., "clap")
- version : The version string (optional, defaults to latest)
- Returns: LLMsStandardConfig object LLMsStandardConfig Type

```ts
type SessionItem = { title: string; description: string; link: string; };
type FullSessionItem = { content: string; link: string; };

export type LLMsStandardConfig = {
  libName: string;
  version: string;
  sessions: SessionItem[];
  fullSessions: FullSessionItem[];
};
```
