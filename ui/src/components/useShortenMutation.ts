import { useMutation } from '@tanstack/react-query';

interface ShortenResponse {
  short_url: string;
  long_url: string;
}

interface ShortenRequest {
  long_url: string;
}

// custom error type
interface ShortenError extends Error {
  status?: number;
  statusText?: string;
  retryAfter?: number;
}

export const useShortenMutation = () => {
  return useMutation<ShortenResponse, ShortenError, ShortenRequest>({
    mutationFn: async ({ long_url }) => {
      const res = await fetch(`/shorten`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ long_url }),
      });

      if (!res.ok) {
        const text = await res.text().catch(() => res.statusText);

        const error: ShortenError = new Error(
          text || `Failed to shorten: ${res.status} ${res.statusText}`
        );
        error.status = res.status;
        error.statusText = res.statusText;
        const retryAfter = res.headers.get('Retry-After');
        if (retryAfter) {
          error.retryAfter = Number(retryAfter);
        }

        throw error;
      }

      return res.json();
    },
  });
};

