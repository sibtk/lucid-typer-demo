import { useState, useCallback } from "react";
import { TextInput } from "./components/dashboard/TextInput";
import { SettingsPanel } from "./components/dashboard/SettingsPanel";
import { ActionBar } from "./components/dashboard/ActionBar";
import { TypingView } from "./components/dashboard/TypingView";
import { CountdownOverlay } from "./components/dashboard/CountdownOverlay";
import { ToastContainer } from "./components/shared/Toast";
import { StartupSequence } from "./components/shared/StartupSequence";
import { Logo } from "./components/shared/Logo";
import { useTypingEngine } from "./hooks/useTypingEngine";

import { useEngineStore } from "./stores/engine-store";
import { useSettingsStore } from "./stores/settings-store";

const orbColors: Record<string, string> = {
  idle: "radial-gradient(circle at 50% 40%, rgba(52,211,153,0.03) 0%, transparent 60%)",
  running: "radial-gradient(circle at 50% 40%, rgba(52,211,153,0.06) 0%, transparent 60%)",
  paused: "radial-gradient(circle at 50% 40%, rgba(251,191,36,0.04) 0%, transparent 60%)",
  completed: "radial-gradient(circle at 50% 40%, rgba(0,212,255,0.04) 0%, transparent 60%)",
  countdown: "radial-gradient(circle at 50% 40%, rgba(52,211,153,0.04) 0%, transparent 60%)",
};

function App() {
  useTypingEngine();

  const status = useEngineStore((s) => s.status);
  const settingsPanelOpen = useSettingsStore((s) => s.settingsPanelOpen);
  const toggleSettingsPanel = useSettingsStore((s) => s.toggleSettingsPanel);

  const [showStartup, setShowStartup] = useState(
    !sessionStorage.getItem("lucid-booted")
  );
  const handleStartupComplete = useCallback(() => setShowStartup(false), []);

  const isTypingMode = status === "running" || status === "paused" || status === "countdown";
  const showTypingView = status === "running" || status === "paused" || status === "countdown";
  const showSettings = !isTypingMode && settingsPanelOpen;

  if (showStartup) {
    return <StartupSequence onComplete={handleStartupComplete} />;
  }

  return (
    <div className="h-screen w-screen bg-bg select-none overflow-hidden relative" data-tauri-drag-region>
      {/* Ambient gradient orb */}
      <div
        className="fixed inset-0 pointer-events-none z-0"
        style={{ background: orbColors[status] || orbColors.idle, transition: "background 2s ease" }}
      />

      {/* Top-edge accent line */}
      <div
        className="absolute top-0 left-0 right-0 h-px z-10 pointer-events-none"
        style={{ background: "linear-gradient(90deg, transparent, rgba(52,211,153,0.3) 50%, transparent)" }}
      />

      {/* Single padded wrapper */}
      <div className="h-full flex flex-col px-8 pt-2 pb-6 relative z-10" data-tauri-drag-region>
        {/* Header */}
        <header
          data-tauri-drag-region
          className="shrink-0 flex items-center justify-between h-10 mb-3 animate-slide-down"
        >
          <div className="flex items-center gap-2" data-tauri-drag-region>
            <div className="relative">
              <Logo size={22} />
              <div
                className="absolute -inset-2 rounded-full blur-[10px] pointer-events-none -z-10"
                style={{ background: "radial-gradient(circle, rgba(52, 211, 153, 0.15) 0%, transparent 70%)" }}
              />
            </div>
            <span className="text-[11px] font-medium tracking-[0.2em] uppercase text-zinc-300">
              Lucid Typer
            </span>
            <span
              className="text-[9px] font-bold uppercase tracking-[0.1em] px-1.5 py-[1px] rounded"
              style={{
                background: "linear-gradient(135deg, rgba(167,139,250,0.15), rgba(139,92,246,0.1))",
                color: "#a78bfa",
                border: "1px solid rgba(167,139,250,0.2)",
              }}
            >
              Pro
            </span>
          </div>

          <div className="flex-1" data-tauri-drag-region />

          <div className="flex items-center gap-3">
            <div className="w-px h-3 bg-white/[0.06]" />
            <div
              className={`w-1.5 h-1.5 rounded-full transition-colors ${
                status === "running" ? "bg-green animate-status-pulse" :
                status === "paused" ? "bg-amber" :
                status === "completed" ? "bg-green" :
                status === "countdown" ? "bg-accent animate-pulse-soft" :
                "bg-white/10"
              }`}
              style={
                status === "running" || status === "completed"
                  ? { boxShadow: "0 0 6px rgba(52, 211, 153, 0.5)", color: "rgba(52, 211, 153, 0.5)" }
                  : status === "paused"
                  ? { boxShadow: "0 0 6px rgba(251, 191, 36, 0.5)" }
                  : undefined
              }
            />
            <span className="text-[11px] text-text-muted capitalize">{status}</span>
          </div>
        </header>

        {/* Body */}
        <div className="flex-1 flex min-h-0 gap-3">
          {/* Center column */}
          <div
            className="flex-1 flex flex-col min-w-0 min-h-0 animate-fade-scale-in"
            style={{ animationDelay: "80ms" }}
          >
            {showTypingView ? <TypingView /> : <TextInput />}
          </div>

          {/* Settings panel — collapsible */}
          {!isTypingMode && (
            <div
              className="shrink-0 overflow-hidden transition-[width] duration-200 ease-out animate-slide-left"
              style={{ width: showSettings ? 240 : 40, animationDelay: "160ms" }}
            >
              {showSettings ? (
                <div className="w-[240px] h-full rounded-xl bg-surface-raised overflow-hidden card-raised flex flex-col">
                  {/* Settings header bar */}
                  <div className="shrink-0 flex items-center justify-between px-4 py-2.5 border-b border-white/[0.06]">
                    <span className="text-[11px] text-text-muted uppercase tracking-[0.15em] font-medium">Settings</span>
                    <button
                      onClick={toggleSettingsPanel}
                      className="text-text-muted hover:text-text-secondary cursor-pointer p-0.5"
                    >
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                        <polyline points="15 18 9 12 15 6" />
                      </svg>
                    </button>
                  </div>
                  <div className="flex-1 min-h-0">
                    <SettingsPanel />
                  </div>
                </div>
              ) : (
                <div className="w-[40px] h-full flex items-start justify-center pt-3">
                  <button
                    onClick={toggleSettingsPanel}
                    className="text-text-muted hover:text-text-secondary cursor-pointer p-2 rounded-lg hover:bg-white/[0.04]"
                    title="Open settings"
                  >
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <circle cx="12" cy="12" r="3" />
                      <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
                    </svg>
                  </button>
                </div>
              )}
            </div>
          )}
        </div>

        {/* Bottom bar */}
        <div className="animate-slide-up" style={{ animationDelay: "240ms" }}>
          <ActionBar />
        </div>
      </div>

      {/* Version */}
      <span className="fixed bottom-2 right-3 text-[9px] text-white/10 font-mono z-10 pointer-events-none">
        v{__APP_VERSION__}
      </span>

      {/* Overlays */}
      <CountdownOverlay />
      <ToastContainer />
    </div>
  );
}

export default App;
