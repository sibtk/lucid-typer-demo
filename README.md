# Lucid Typer (Demo)

Human typing simulation desktop app. Paste any text and Lucid Typer will type it out keystroke-by-keystroke with realistic human behaviors — errors, corrections, pauses, fatigue, and more.

## Download

Grab the latest `.dmg` from [Releases](https://github.com/sibtk/lucid-typer-demo/releases) (macOS Apple Silicon).

## Features

- **16+ typing behavior rules** — digraph timing, word boundaries, punctuation pauses, paragraph breaks, thinking pauses, burst mode, fatigue, rollover, micro-corrections, second thoughts, error clustering, and more
- **Realistic errors & corrections** — substitution, insertion, omission, double letters, transposition, wrong caps — each with natural correction behavior
- **AI Humanizer** — rewrite text to bypass AI detection before typing (requires API)
- **Personality presets** — switch between typing styles
- **Live preview** — watch characters appear with real-time WPM and error tracking
- **Fully configurable** — tune every behavior, pause range, and error weight

## Stack

- **Desktop**: Tauri v2 (Rust backend, React frontend)
- **Frontend**: React 19, TypeScript, Tailwind v4, Zustand
- **Backend**: Rust with enigo for native key injection

## Development

```bash
pnpm install
pnpm tauri dev
```

## Build

```bash
pnpm tauri build --target aarch64-apple-darwin
```

Output: `src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/`

## Humanizer API

The humanizer connects to an external API via `VITE_API_BASE_URL` (defaults to `http://localhost:3000`). Set the env var to point at your humanizer backend:

```bash
VITE_API_BASE_URL=https://your-api.com pnpm tauri dev
```
