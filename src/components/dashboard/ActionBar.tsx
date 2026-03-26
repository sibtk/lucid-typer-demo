import { useState, useEffect, useRef } from "react";
import { useEngineStore } from "../../stores/engine-store";
import { useSettingsStore } from "../../stores/settings-store";
import { startTyping, stopTyping, pauseTyping, resumeTyping, estimateTypingTime } from "../../lib/tauri-commands";
import { getPersonality } from "../../lib/personalities";
import { useToastStore } from "../../stores/toast-store";
import { useHumanizerStore } from "../../stores/humanizer-store";
import type { EstimateResult } from "../../lib/tauri-commands";

export function ActionBar() {
  const { status, inputText, reset, currentWpm, errorCount, typedChars, totalChars, elapsedMs, progress } = useEngineStore();
  const settings = useSettingsStore();
  const addToast = useToastStore((s) => s.addToast);
  const personality = getPersonality(settings.activePersonality);
  const isIdle = status === "idle" || status === "completed";
  const [estimate, setEstimate] = useState<EstimateResult | null>(null);

  const buildConfig = () => {
    const enabledTypes = [
      settings.substitutionEnabled ? 0.45 : 0,
      settings.insertionEnabled ? 0.18 : 0,
      settings.omissionEnabled ? 0.17 : 0,
      settings.doubleLetterEnabled ? 0.12 : 0,
      settings.transpositionEnabled ? 0.08 : 0,
      settings.wrongCapsEnabled ? 0.05 : 0,
    ];
    const total = enabledTypes.reduce((a, b) => a + b, 0) || 1;

    return {
      wpm: settings.customWpm,
      speed_variation: settings.speedVariation,
      digraph_enabled: true,
      modifiers_enabled: true,
      word_boundary_enabled: true,
      punctuation_pauses_enabled: settings.punctuationPausesEnabled,
      paragraph_pause: settings.paragraphPause,
      thinking_pause: {
        enabled: settings.thinkingPausesEnabled,
        frequency: settings.thinkingFrequency,
        preset: settings.thinkingPreset,
      },
      error_config: {
        enabled: settings.errorRate > 0,
        error_rate: settings.errorRate,
        substitution_weight: enabledTypes[0] / total,
        insertion_weight: enabledTypes[1] / total,
        omission_weight: enabledTypes[2] / total,
        double_letter_weight: enabledTypes[3] / total,
        transposition_weight: enabledTypes[4] / total,
        wrong_caps_weight: enabledTypes[5] / total,
      },
      correction_config: {
        enabled: settings.correctionEnabled,
        speed: settings.correctionSpeed,
        over_backspace_chance: settings.overBackspaceChance,
      },
      rollover_enabled: settings.rolloverEnabled,
      rollover_chance: personality.rolloverChance,
      fatigue_config: {
        enabled: settings.fatigueEnabled,
        onset_minutes: 15 + personality.fatigueResistance * 10,
      },
      micro_correction: {
        enabled: settings.microCorrectionsEnabled,
        chance: settings.microCorrectionChance,
        min_chars_between: 18,
      },
      second_thoughts: {
        enabled: settings.secondThoughtsEnabled,
        chance: settings.secondThoughtsChance,
        min_words_between: 6,
        synonym_chance: settings.synonymChance,
      },
      burst_pause: settings.burstMode
        ? { enabled: true, min_burst_length: 5, max_burst_length: 20, min_pause_ms: 200, max_pause_ms: 800 }
        : null,
      error_clustering: {
        enabled: settings.errorClusteringEnabled,
        multiplier: 2.5,
        range: 7,
      },
      word_substitution: {
        enabled: settings.wordSubstitutionEnabled,
        chance: settings.wordSubstitutionChance,
        partial_chars: 4,
        min_words_between: 8,
      },
      hesitation_backspace: {
        enabled: settings.hesitationBackspaceEnabled,
        chance: settings.hesitationBackspaceChance,
        min_chars_between: 25,
      },
      mid_word_pause: {
        enabled: settings.midWordPauseEnabled,
        chance: settings.midWordPauseChance,
        min_chars_between: 30,
        pause_ms_min: 300,
        pause_ms_max: 1500,
      },
      sentence_restart: {
        enabled: settings.sentenceRestartEnabled,
        chance: settings.sentenceRestartChance,
        min_sentences_between: 3,
      },
      punctuation_pause_config: {
        enabled: settings.punctuationPausesEnabled,
        period_ms: settings.periodPause,
        comma_ms: settings.commaPause,
        question_ms: settings.questionPause,
        exclamation_ms: settings.exclamationPause,
        colon_ms: settings.colonPause,
        semicolon_ms: settings.semicolonPause,
      },
      unfamiliar_word_slowdown: settings.unfamiliarWordSlowdown,
      unfamiliar_word_multiplier: 1.5,
      auto_pause: {
        enabled: settings.autoPauseEnabled,
        interval_minutes: settings.autoPauseInterval,
        duration_minutes: settings.autoPauseDuration,
      },
    };
  };

  // Debounced time estimate
  useEffect(() => {
    if (!isIdle || !inputText.trim()) {
      setEstimate(null);
      return;
    }
    const timer = setTimeout(async () => {
      try {
        const result = await estimateTypingTime(inputText, buildConfig());
        setEstimate(result);
      } catch {
        setEstimate(null);
      }
    }, 500);
    return () => clearTimeout(timer);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [inputText, settings.customWpm, settings.errorRate, settings.speedVariation, isIdle]);

  const handleStart = async () => {
    if (!inputText.trim()) return;
    reset();
    useEngineStore.getState().setStatus("countdown");
    try {
      await startTyping(inputText, buildConfig(), "inject", 5);
    } catch (e) {
      console.error("Failed to start:", e);
      useEngineStore.getState().setStatus("idle");
    }
  };

  const handleStop = async () => {
    await stopTyping();
    useEngineStore.getState().setStatus("idle");
    addToast("Stopped", "info");
  };

  const handlePause = async () => {
    await pauseTyping();
    useEngineStore.getState().setStatus("paused");
  };

  const handleResume = async () => {
    await resumeTyping();
    useEngineStore.getState().setStatus("running");
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Enter" && (e.metaKey || e.ctrlKey) && isIdle && inputText.trim()) {
        e.preventDefault();
        handleStart();
      } else if (e.key === "Escape" && (status === "running" || status === "paused" || status === "countdown")) {
        e.preventDefault();
        handleStop();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [status, inputText]);

  // Humanizer state
  const humanizer = useHumanizerStore();
  const [showModeMenu, setShowModeMenu] = useState(false);
  const modeMenuRef = useRef<HTMLDivElement>(null);
  const isHumanizing = humanizer.status === "running";
  const humanizeDone = humanizer.status === "done";

  // Close mode menu on outside click
  useEffect(() => {
    if (!showModeMenu) return;
    const handler = (e: MouseEvent) => {
      if (modeMenuRef.current && !modeMenuRef.current.contains(e.target as Node)) {
        setShowModeMenu(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [showModeMenu]);

  const handleHumanize = () => {
    if (!inputText.trim() || isHumanizing) return;
    humanizer.start(inputText, (humanizedText) => {
      useEngineStore.getState().setInputText(humanizedText);
    });
  };

  const handleUndoHumanize = () => {
    humanizer.reset();
  };

  const modeLabels = { light: "Light", balanced: "Balanced", aggressive: "Aggressive" } as const;

  const hasText = !!inputText.trim();
  const progressPct = Math.round(progress * 100);

  const getTimeRemaining = () => {
    if (typedChars <= 0 || elapsedMs <= 0) return null;
    const msPerChar = elapsedMs / typedChars;
    const remainingChars = totalChars - typedChars;
    const remainingMs = msPerChar * remainingChars;
    const totalSec = Math.ceil(remainingMs / 1000);
    const m = Math.floor(totalSec / 60);
    const s = totalSec % 60;
    return `~${m}:${s.toString().padStart(2, "0")}`;
  };

  const formatEstimate = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = Math.round(seconds % 60);
    if (m === 0) return `~${s}s`;
    if (s === 0) return `~${m}m`;
    return `~${m}m ${s}s`;
  };

  return (
    <div className="shrink-0 min-h-[44px] py-2 flex items-center justify-between min-w-0 overflow-visible">
      {isIdle && !isHumanizing ? (
        <>
          <div className="flex items-center gap-2.5">
            {/* Start typing button — left */}
            <button
              onClick={handleStart}
              disabled={!hasText}
              className={`h-8 px-4 rounded-lg text-[13px] font-medium cursor-pointer
                disabled:opacity-20 disabled:cursor-not-allowed
                text-black hover:brightness-110 active:brightness-95 ${hasText ? "glow-breathe" : ""}`}
              style={{
                background: hasText
                  ? "linear-gradient(to bottom, #34d399, #28b87d)"
                  : "#34d399",
                boxShadow: hasText
                  ? "inset 0 1px 0 rgba(255,255,255,0.15), 0 2px 8px rgba(52,211,153,0.2)"
                  : undefined,
              }}
            >
              Start typing
            </button>

            {/* Humanize button + mode selector */}
            <div className="relative flex items-center" ref={modeMenuRef}>
              <button
                onClick={handleHumanize}
                disabled={!hasText}
                className={`h-8 px-3.5 rounded-l-lg text-[12px] font-semibold uppercase tracking-[0.05em] cursor-pointer
                  disabled:opacity-20 disabled:cursor-not-allowed
                  hover:brightness-125 active:brightness-95`}
                style={{
                  background: "linear-gradient(135deg, rgba(167,139,250,0.15), rgba(139,92,246,0.1))",
                  color: "#a78bfa",
                  border: "1px solid rgba(167,139,250,0.2)",
                  borderRight: "none",
                  boxShadow: hasText
                    ? "0 0 12px rgba(167,139,250,0.1), 0 0 4px rgba(167,139,250,0.05)"
                    : undefined,
                }}
              >
                Humanize
              </button>
              <button
                onClick={() => setShowModeMenu(!showModeMenu)}
                disabled={!hasText}
                className="h-8 px-1.5 rounded-r-lg text-[12px] cursor-pointer
                  disabled:opacity-20 disabled:cursor-not-allowed"
                style={{
                  background: "linear-gradient(135deg, rgba(167,139,250,0.15), rgba(139,92,246,0.1))",
                  color: "#a78bfa",
                  border: "1px solid rgba(167,139,250,0.2)",
                  borderLeft: "1px solid rgba(167,139,250,0.15)",
                }}
              >
                <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M6 9l6 6 6-6" />
                </svg>
              </button>

              {showModeMenu && (
                <div className="absolute bottom-full left-0 mb-1 z-50 rounded-lg border border-[rgba(167,139,250,0.2)] bg-[#1a1a2e] shadow-xl py-1 min-w-[140px]">
                  {(["light", "balanced", "aggressive"] as const).map((m) => (
                    <button
                      key={m}
                      onClick={() => { humanizer.setMode(m); setShowModeMenu(false); }}
                      className={`w-full text-left px-3 py-1.5 text-[12px] hover:bg-white/5 ${
                        humanizer.mode === m ? "text-[#a78bfa] font-medium" : "text-white/70"
                      }`}
                    >
                      {modeLabels[m]}
                    </button>
                  ))}
                </div>
              )}
            </div>

            {/* Score badge */}
            {humanizeDone && humanizer.score != null && (
              <div className="flex items-center gap-1.5">
                <span className={`text-[12px] font-mono font-semibold ${
                  humanizer.score >= 80 ? "text-[#34d399]" : humanizer.score >= 50 ? "text-amber" : "text-red"
                }`}>
                  {humanizer.score}% human
                </span>
                <button
                  onClick={handleUndoHumanize}
                  className="text-[10px] text-text-muted hover:text-white/80 underline underline-offset-2"
                >
                  undo
                </button>
              </div>
            )}
          </div>

          <div className="flex items-center gap-4 text-[11px] text-text-muted min-w-0">
            <span className="truncate shrink-0">{personality.icon} {personality.name}</span>
            <span className="font-mono shrink-0">{settings.customWpm} wpm</span>
            {estimate && (
              <span className="font-mono">{formatEstimate(estimate.total_seconds)} est</span>
            )}
          </div>
        </>
      ) : isHumanizing ? (
        <>
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 border-2 border-[#a78bfa]/30 border-t-[#a78bfa] rounded-full animate-spin" />
              <span className="text-[13px] text-[#a78bfa] font-medium">Humanizing...</span>
            </div>
            <span className="text-[11px] text-text-muted">{humanizer.statusMessage}</span>
          </div>
          <button
            onClick={() => humanizer.reset()}
            className="h-8 px-4 rounded-lg text-[13px] font-medium cursor-pointer
              text-text-muted hover:text-red hover:bg-red/5"
          >
            Cancel
          </button>
        </>
      ) : (
        <>
          <div className="flex items-center gap-2">
            {status === "running" ? (
              <button
                onClick={handlePause}
                className="h-8 px-4 rounded-lg text-[13px] font-medium cursor-pointer
                  bg-white/10 text-text-primary hover:bg-white/15"
              >
                Pause
              </button>
            ) : status === "paused" ? (
              <button
                onClick={handleResume}
                className="h-8 px-4 rounded-lg text-[13px] font-medium cursor-pointer
                  text-black hover:brightness-110 glow-breathe"
                style={{
                  background: "linear-gradient(to bottom, #34d399, #28b87d)",
                  boxShadow: "inset 0 1px 0 rgba(255,255,255,0.15), 0 2px 8px rgba(52,211,153,0.2)",
                }}
              >
                Resume
              </button>
            ) : null}
            <button
              onClick={handleStop}
              className="h-8 px-4 rounded-lg text-[13px] font-medium cursor-pointer
                text-text-muted hover:text-red hover:bg-red/5"
            >
              Stop
            </button>
          </div>

          {(status === "running" || status === "paused") && (
            <div className="flex items-center gap-3 text-[11px] min-w-0">
              <div className="flex items-center gap-1.5 shrink-0">
                <span className="font-mono text-[14px] text-white font-medium">{currentWpm}</span>
                <span className="text-text-muted">wpm</span>
              </div>
              <span className="text-text-secondary shrink-0">{errorCount} errors</span>
              {getTimeRemaining() && (
                <span className="text-text-muted font-mono shrink-0">{getTimeRemaining()} left</span>
              )}
              <span className="text-text-muted font-mono shrink-0">{progressPct}%</span>
            </div>
          )}
        </>
      )}
    </div>
  );
}
