import { getRunEnv, RUN_ENV, invokeTauri, logger } from "@/adapters";
import {HttpFetchOptions, HttpFetchResponse} from "@/lib/types";

export const httpFetch = async (
  url: string,
  options?: HttpFetchOptions,
): Promise<HttpFetchResponse> => {
  // Network fetch is only available in desktop mode for security reasons
  if (getRunEnv() !== RUN_ENV.DESKTOP) {
    throw new Error("Network fetch is only available in desktop mode");
  }

  try {
    return await invokeTauri("http_fetch", { url, options });
  } catch (error) {
    logger.error(`Error fetching ${url}`);
    throw error;
  }
};
