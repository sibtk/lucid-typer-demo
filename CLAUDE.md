# Lucid Typer

Human typing simulation desktop app. Tauri v2 + React + Rust.

## Stack
- **Desktop**: Tauri v2 (Rust backend, React frontend)
- **Frontend**: React 19, TypeScript, Tailwind v4, Zustand, Zod
- **Backend**: Rust with enigo for key injection
- **Auth**: Supabase (magic link)
- **Payments**: Stripe + BTCPay Server

## Commands
- `pnpm dev` — Start Vite dev server only
- `pnpm tauri dev` — Start full Tauri dev (frontend + Rust backend)
- `pnpm build` — Build frontend
- `pnpm tauri build` — Build distributable app
- `cargo test --manifest-path src-tauri/Cargo.toml` — Run Rust tests

## Architecture
- `src/` — React frontend (components, stores, hooks, styles)
- `src-tauri/src/engine/` — Rust typing engine (core IP)
  - `core.rs` — Main orchestrator loop
  - `timing.rs` — Delay calculations (Rules 1-8)
  - `errors.rs` — Error injection + correction (Rules 11-12)
  - `behaviors.rs` — Rollover, fatigue, micro-corrections, second thoughts (Rules 13-16)
  - `pauses.rs` — Paragraph, thinking, inline pauses (Rules 9-10, 17)
  - `keyboard_map.rs` — QWERTY layout data
  - `digraph.rs` — Letter pair timing
- `src-tauri/src/commands/` — Tauri command handlers

## Design System
- Dark glassmorphic aesthetic
- Multi-color glow: cyan=stats, purple=settings, green=active, amber=warnings, pink=personalities, red=errors
- Status-based ambient glow shifts during typing
