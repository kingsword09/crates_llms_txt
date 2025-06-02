import { get_llms_config_online } from "./crates_llms_txt.js";

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
 * @module
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
