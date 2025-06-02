import { get_llms_config } from "./crates_llms_txt.js";

type SessionItem = { title: string; description: string; link: string; };

type FullSessionItem = { content: string; link: string; };

type LLMsStandardStringConfig = { lib_name: string; version: string; sessions: string; full_sessions: string; };

/**
 * The standard configuration for LLMs
 */
export type LLMsStandardConfig = {
  libName: string;
  version: string;
  sessions: SessionItem[];
  fullSessions: FullSessionItem[];
};

/**
 * Get the standard configuration for LLMs based on library name and version
 *
 * @param lib_name - The name of the library
 * @param version - The version of the library
 * @returns
 */
export const get_llms_standard_config = async (lib_name: string, version?: string): Promise<LLMsStandardConfig> => {
  const config: LLMsStandardStringConfig = await get_llms_config(lib_name, version);

  return {
    libName: config.lib_name,
    version: config.version,
    sessions: JSON.parse(config.sessions),
    fullSessions: JSON.parse(config.full_sessions),
  } satisfies LLMsStandardConfig;
};
