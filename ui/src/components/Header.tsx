import { version } from '../../package.json';

export default function Header() {
  return (
    <header className="mb-auto">
      <div>
        <h1 className="mb-0 text-xl">
          🐦 Tui <span className="mx-1" aria-hidden>⟶</span> li
        </h1>
        <p className="text-xs opacity-70">v{version}</p>
        <p className="text-sm opacity-80">
          Your friendly URL shortener from NZ
        </p>
      </div>
    </header>
  );
}
