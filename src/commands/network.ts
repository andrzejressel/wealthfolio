import { getRunEnv, RUN_ENV, invokeTauri, logger } from "@/adapters";

export interface FetchOptions {
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH' | 'HEAD' | 'OPTIONS';
  headers?: Record<string, string>;
  body?: Uint8Array;
}

export interface FetchResponse {
  status: number;
  headers: Record<string, string>;
  body: Uint8Array;
}

export const httpFetch = async (
  url: string,
  options?: FetchOptions,
): Promise<FetchResponse> => {
  try {
    // Network fetch is only available in desktop mode for security reasons
    if (getRunEnv() !== RUN_ENV.DESKTOP) {
      throw new Error("Network fetch is only available in desktop mode");
    }

    return await invokeTauri("http_fetch", { url, options });
  } catch (error) {
    logger.error(`Error fetching ${url}`);
    throw error;
  }
};
