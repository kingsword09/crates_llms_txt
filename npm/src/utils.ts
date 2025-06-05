import type { LlMsConfig } from "crates-llms-txt-napi";

import type { FullSessionItem, LLMsStandardConfig, SessionItem } from "./types.ts";

/**
 * Merge sessions with duplicate links.
 *
 * @param sessions - The array of SessionItem objects.
 * @returns
 */
export const mergeSessions = (sessions: SessionItem[]): SessionItem[] => {
  // 1. Group by link (keep the last occurrence)
  const linkMap = new Map<string, SessionItem>();
  for (const session of sessions) {
    linkMap.set(session.link, { ...session }); // Shallow copy to avoid modifying original object
  }

  // 2. Process each unique link
  return Array.from(linkMap.values()).map((item) => ({
    ...item,
    title: extractFilenameWithoutExtension(item.link),
  }));
};

const extractFilenameWithoutExtension = (link: string): string => {
  const segments = link.split("/").filter((s) => s !== "");
  let filename = segments.pop() || link;

  const lastDotIndex = filename.lastIndexOf(".");
  return lastDotIndex > 0 ? filename.substring(0, lastDotIndex) : filename;
};

/**
 * Merge full sessions with duplicate links by combining their content.
 * If multiple sessions share the same link, their content will be concatenated
 * with double newlines as separators.
 *
 * @param sessions - The array of FullSessionItem objects.
 * @returns The array of merged FullSessionItem objects.
 */
export const mergeFullSessionsOptimized = (
  sessions: FullSessionItem[]
): FullSessionItem[] => {
  // Create a Map to store unique links and their combined content
  const linkMap = new Map<string, string>();

  // Iterate through sessions and merge content for duplicate links
  for (const session of sessions) {
    const existing = linkMap.get(session.link);
    // If link exists, append new content with double newline separator
    // Otherwise use the content directly
    linkMap.set(
      session.link,
      existing ? `${existing}\n\n${session.content}` : session.content
    );
  }

  // Convert Map entries back to FullSessionItem array
  return Array.from(linkMap.entries()).map(([link, content]) => ({
    link,
    content,
  }));
};

/**
 * Process the LlMsConfig object and return a LLMsStandardConfig object.
 *
 * @param config - The LlMsConfig object.
 * @returns The LLMsStandardConfig object or null if an error occurs.
 */
export const processLlmsConfig = (
  config: LlMsConfig | null
): LLMsStandardConfig | null => {
  if (!config) return null;

  const parsedSessions: SessionItem[] = JSON.parse(config.sessions);
  const sessions = mergeSessions(parsedSessions);

  const parsedFullSessions: FullSessionItem[] = JSON.parse(config.fullSessions);
  const fullSessions = mergeFullSessionsOptimized(parsedFullSessions);

  try {
    return {
      libName: config.libName,
      version: config.version,
      sessions: sessions,
      fullSessions: fullSessions,
    } satisfies LLMsStandardConfig;
  } catch (_err) {
    return null;
  }
};
