/**
 * @module
 *
 * This module provides functions for getting the standard configuration for LLMs.
 */

import {
  get_llms_config_by_rustdoc_all_features,
  get_llms_config_by_rustdoc_features,
  get_llms_config_online,
} from "./crates_llms_txt.js";

type SessionItem = { title: string; description: string; link: string; };

type FullSessionItem = { content: string; link: string; };

type LLMsStandardStringConfig = { lib_name: string; version: string; sessions: string; full_sessions: string; };

/**
 * The standard configuration for LLMs
 */
export type LLMsStandardConfig = {
  libName: string;
  version: string;
  sessions?: SessionItem[];
  fullSessions?: FullSessionItem[];
};

/**
 * Get the standard configuration for LLMs based on library name and version
 *
 * @param lib_name - The name of the library
 * @param version - The version of the library
 * @returns LLMsStandardConfig - The standard configuration for LLMs
 *
 * @example
 * ```ts
 * import { get_llms_standard_config } from "jsr:@llms/llms@^0.1.0";
 *
 * const llmsStandardConfig = await get_llms_standard_config("clap", "4.5.39");
 * ```
 */
export const get_llms_standard_config_by_net = async (
  lib_name: string,
  version?: string,
): Promise<LLMsStandardConfig | null> => {
  const config: LLMsStandardStringConfig = await get_llms_config_online(lib_name, version);

  if (!config) {
    return null;
  }

  return {
    libName: config.lib_name,
    version: config.version,
    sessions: config.sessions ? JSON.parse(config.sessions) : undefined,
    fullSessions: config.full_sessions ? JSON.parse(config.full_sessions) : undefined,
  } satisfies LLMsStandardConfig;
};

/**
 * Get the standard configuration for LLMs based on toolchain, manifest path, and features
 *
 * @param toolchain - The toolchain to use
 * @param manifest_path - The path to the manifest file
 * @returns LLMsStandardConfig - The standard configuration for LLMs
 *
 * @example
 *
 * ```ts
 * import { get_llms_standard_config_by_rustdoc_all_features } from "jsr:@llms/llms@^0.1.0";
 *
 * const llmsStandardConfig = await get_llms_standard_config_by_rustdoc_all_features("nightly", "Cargo.toml");
 * ```
 */
export const get_llms_standard_config_by_rustdoc_all_features = async (
  toolchain: string,
  manifest_path: string,
): Promise<LLMsStandardConfig | null> => {
  const config: LLMsStandardStringConfig = await get_llms_config_by_rustdoc_all_features(toolchain, manifest_path);

  if (!config) {
    return null;
  }

  return {
    libName: config.lib_name,
    version: config.version,
    sessions: config.sessions ? JSON.parse(config.sessions) : undefined,
    fullSessions: config.full_sessions ? JSON.parse(config.full_sessions) : undefined,
  } satisfies LLMsStandardConfig;
};

/**
 * Get the standard configuration for LLMs based on toolchain, manifest path, features, and no default features
 *
 * @param toolchain - The toolchain to use
 * @param manifest_path - The path to the manifest file
 * @param no_default_features - Whether to include default features
 * @param features - The features to include
 * @returns LLMsStandardConfig - The standard configuration for LLMs
 *
 * @example
 *
 * ```ts
 * import { get_llms_standard_config_by_rustdoc_features } from "jsr:@llms/llms@^0.1.0";
 *
 * const llmsStandardConfig = await get_llms_standard_config_by_rustdoc_features("nightly", "Cargo.toml", true, ["async"]);
 * ```
 */
export const get_llms_standard_config_by_rustdoc_features = async (
  toolchain: string,
  manifest_path: string,
  no_default_features: boolean,
  features?: string[],
): Promise<LLMsStandardConfig | null> => {
  const config: LLMsStandardStringConfig = await get_llms_config_by_rustdoc_features(
    toolchain,
    manifest_path,
    no_default_features,
    features,
  );

  if (!config) {
    return null;
  }

  return {
    libName: config.lib_name,
    version: config.version,
    sessions: config.sessions ? JSON.parse(config.sessions) : undefined,
    fullSessions: config.full_sessions ? JSON.parse(config.full_sessions) : undefined,
  } satisfies LLMsStandardConfig;
};
