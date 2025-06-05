/**
 * SessionItem is the session item.
 * @property title - The title of the session.
 * @property description - The description of the session.
 * @property link - The link of the session.
 */
export type SessionItem = {
  title: string;
  description: string;
  link: string;
};

/**
 * FullSessionItem is the full session item.
 * @property content - The content of the session.
 * @property link - The link of the session.
 */
export type FullSessionItem = {
  content: string;
  link: string;
};

/**
 * LLMsStandardConfig is the standard config for LLMs.
 * @property libName - The name of the library.
 * @property version - The version of the library.
 * @property sessions - The sessions of the library.
 * @property fullSessions - The full sessions of the library.
 */
export type LLMsStandardConfig = {
  libName: string;
  version: string;
  sessions: SessionItem[];
  fullSessions: FullSessionItem[];
};
