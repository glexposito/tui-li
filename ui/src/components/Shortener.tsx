import { useState } from 'react';
import { useShortenMutation } from './useShortenMutation';
import isURL from "validator/lib/isURL";
import styles from './Shortener.module.css';

/** Strip leading "www." from a URL origin (keeps protocol/port; localhost unaffected) */
function canonicalizeOrigin(raw: string) {
  try {
    const u = new URL(raw);
    if (u.hostname.startsWith('www.')) {
      u.hostname = u.hostname.slice(4);
    }
    return u.origin;
  } catch {
    return raw;
  }
}

/** One consistent error wrapper: same icon + classes; only message text varies */
function ErrorMessage({
  clientError,
  error,
}: {
  clientError?: string | null;
  error?: any; // (use your ShortenError type here if exported)
}) {
  let message: string | null = null;

  if (clientError) {
    message = clientError;
  } else if (error) {
    const status: number | undefined = error?.status;
    const retryAfter: number | undefined = error?.retryAfter;

    if (status === 429) {
      message = `Whoa there, turbo! You’ve hit the rate limit.${retryAfter != null
        ? ` Try again in about ${retryAfter} second${retryAfter === 1 ? '' : 's'}.`
        : ' Take a quick breather and try again.'
      }`;
    } else if (status === 400 || status === 422) {
      message = 'The URL you entered doesn’t look valid. Please double-check and try again.';
    } else if (status === 500) {
      message = 'Oops, something went wrong on our servers. Please try again later.';
    } else if (status === 0) {
      message = 'Network error. Please check your connection and try again.';
    } else {
      message = 'Well this is embarrassing, there was an error shortening your URL.';
    }
  }

  if (!message) return null;

  return (
    <p className="lead text-main mt-3">
      <i className="bi bi-emoji-frown-fill px-2 text-danger" />
      {message}
    </p>
  );
}

/** Success block (short link + copy) */
function ShortLink({
  id,
  origin,
  copied,
  onCopy,
}: {
  id: string;
  origin: string;
  copied: boolean;
  onCopy: () => void;
}) {
  const href = `${origin}/${id}`;
  return (
    <div className="text-center d-flex justify-content-center align-items-center gap-2">
      <span className="lead text-main">Short link:</span>
      <a href={href} target="_blank" rel="noreferrer" className="fs-5 text-decoration-none">
        {href}
      </a>
      <button
        className="btn btn-sm d-flex align-items-center"
        style={{ background: 'transparent', border: 'none', color: 'white' }}
        onClick={onCopy}
        aria-label="Copy short link"
        title={copied ? 'Copied!' : 'Copy'}
      >
        <i className={`bi ${copied ? 'bi-clipboard-check' : 'bi-clipboard'}`} />
      </button>
    </div>
  );
}

export default function Shortener() {
  const shortenMutation = useShortenMutation();

  const [url, setUrl] = useState('');
  const [copied, setCopied] = useState(false);
  const [clientError, setClientError] = useState<string | null>(null);

  const origin =
    typeof window !== 'undefined' ? canonicalizeOrigin(window.location.origin) : 'https://tuili.kiwi';

  const handleShorten = () => {
    setClientError(null);

    let trimmed = url.trim();
    if (!trimmed) return;

    // Prepend https:// if no scheme
    if (!/^[a-zA-Z][a-zA-Z\d+\-.]*:\/\//.test(trimmed)) {
      trimmed = `https://${trimmed}`;
    }

    // ✅ Validate early with validator.js
    const ok = isURL(trimmed, {
      require_protocol: true,
      protocols: ["http", "https"], // only allow http/https
      allow_underscores: false,
      allow_trailing_dot: false,
      allow_protocol_relative_urls: false,
    });

    if (!ok) {
      setClientError("Please enter a valid URL (e.g. https://example.com).");
      return;
    }

    setCopied(false);
    shortenMutation.reset();
    shortenMutation.mutate({ long_url: trimmed });
  };

  const handleCopy = async () => {
    if (!shortenMutation.data) return;
    try {
      await navigator.clipboard.writeText(`${origin}/${shortenMutation.data.id}`);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      setCopied(false);
      // optional: show a non-blocking toast/snackbar
      console.warn('Clipboard write failed; user can copy manually.');
    }
  };

  const showSlot =
    !!clientError || shortenMutation.isSuccess || shortenMutation.isError;

  return (
    <div className="App">
      <main className="px-3">
        {/* URL shortener form */}
        <div className="mt-4">
          <div className="input-group">
            <input
              type="url"
              className="form-control"
              placeholder="Enter a full URL"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleShorten()}
            />
            <button type="button" className="btn btn-primary" onClick={handleShorten}>
              ✨ Shorten URL
            </button>
          </div>

          {/* Single reserved slot for result OR error */}
          <div
            className={`${styles.fadeSlot} mt-4 ${showSlot ? styles.show : ''}`}
            aria-live="polite"
          >
            {clientError ? (
              <ErrorMessage clientError={clientError} />
            ) : shortenMutation.isSuccess ? (
              <ShortLink
                id={shortenMutation.data!.id}
                origin={origin}
                copied={copied}
                onCopy={handleCopy}
              />
            ) : shortenMutation.isError ? (
              <ErrorMessage error={shortenMutation.error as any} />
            ) : null}
          </div>
        </div>

        {/* Tagline */}
        <ul className="mt-5 text-center text-white fs-6 list-unstyled">
          <li>Fast & reliable shortener — no ads</li>
          <li>Free, and highly available</li>
          <li>Short & sweet links for the web</li>
        </ul>
      </main>
    </div>
  );
}
