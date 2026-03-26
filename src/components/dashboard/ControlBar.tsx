import { useEngineStore } from "../../stores/engine-store";
import { useSettingsStore } from "../../stores/settings-store";
import { startTyping, stopTyping, pauseTyping, resumeTyping } from "../../lib/tauri-commands";
import { getPersonality, personalities } from "../../lib/personalities";
import type { CorrectionSpeed, ParagraphPause } from "../../stores/settings-store";

export function ControlBar() {
  const { status, inputText, reset } = useEngineStore();
  const settings = useSettingsStore();
  const personality = getPersonality(settings.activePersonality);
  const isIdle = status === "idle" || status === "completed";

  const buildConfig = () => {
    // Compute error type weights from toggles
    const enabledTypes = [
      settings.substitutionEnabled ? 0.45 : 0,
      settings.insertionEnabled ? 0.18 : 0,
      settings.omissionEnabled ? 0.17 : 0,
      settings.doubleLetterEnabled ? 0.12 : 0,
      settings.transpositionEnabled ? 0.08 : 0,
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
        preset: "Normal",
      },
      error_config: {
        enabled: settings.errorRate > 0,
        error_rate: settings.errorRate,
        substitution_weight: enabledTypes[0] / total,
        insertion_weight: enabledTypes[1] / total,
        omission_weight: enabledTypes[2] / total,
        double_letter_weight: enabledTypes[3] / total,
        transposition_weight: enabledTypes[4] / total,
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
      },
      burst_pause: settings.burstMode
        ? {
            enabled: true,
            min_burst_length: 5,
            max_burst_length: 20,
            min_pause_ms: 200,
            max_pause_ms: 800,
          }
        : null,
      error_clustering: {
        enabled: settings.errorClusteringEnabled,
        multiplier: 2.5,
        range: 7,
      },
    };
  };

  const handleStart = async () => {
    if (!inputText.trim()) return;
    reset();
    useEngineStore.getState().setStatus("countdown");
    try {
      await startTyping(inputText, buildConfig(), "inject", 5);
    } catch (e) {
      console.error("Failed to start typing:", e);
      useEngineStore.getState().setStatus("idle");
    }
  };

  const handlePreview = async () => {
    if (!inputText.trim()) return;
    reset();
    useEngineStore.getState().setStatus("running");
    try {
      await startTyping(inputText, buildConfig(), "preview", 0);
    } catch (e) {
      console.error("Failed to preview:", e);
      useEngineStore.getState().setStatus("idle");
    }
  };

  const handlePause = async () => {
    await pauseTyping();
    useEngineStore.getState().setStatus("paused");
  };

  const handleResume = async () => {
    await resumeTyping();
    useEngineStore.getState().setStatus("running");
  };

  const handleStop = async () => {
    await stopTyping();
    useEngineStore.getState().setStatus("idle");
  };

  return (
    <div className="glass p-4 flex flex-col gap-3">
      {/* Personality pills */}
      {isIdle && (
        <>
          <div className="flex items-center gap-2 flex-wrap">
            <span className="text-[10px] text-text-muted uppercase tracking-wider shrink-0">Style</span>
            {personalities.map((p) => (
              <button
                key={p.id}
                onClick={() => {
                  settings.setPersonality(p.id);
                  settings.setWpm(p.wpm);
                  settings.setErrorRate(p.errorRate);
                  settings.setSpeedVariation(p.speedVariation);
                  settings.setBurstMode(p.burstMode);
                  settings.setRollover(p.rolloverEnabled);
                  settings.setFatigue(p.fatigueResistance < 0.5);
                  settings.setThinkingFrequency(p.thinkingFrequency);
                }}
                className={`px-2.5 py-1 rounded-full text-xs transition-all cursor-pointer ${
                  settings.activePersonality === p.id
                    ? "bg-glow-cyan/20 text-glow-cyan border border-glow-cyan/40"
                    : "bg-white/5 text-text-secondary border border-transparent hover:bg-white/8 hover:text-text-primary"
                }`}
              >
                {p.icon} {p.name}
              </button>
            ))}
          </div>

          {/* Speed + Error rate + Variation sliders */}
          <div className="flex gap-4">
            <div className="flex-1">
              <div className="flex items-center justify-between mb-1">
                <span className="text-[10px] text-text-muted uppercase tracking-wider">Speed</span>
                <span className="text-xs font-mono text-glow-cyan">{settings.customWpm} WPM</span>
              </div>
              <input
                type="range" min="15" max="150"
                value={settings.customWpm}
                onChange={(e) => settings.setWpm(parseInt(e.target.value))}
                className="w-full accent-glow-cyan h-1"
              />
            </div>
            <div className="flex-1">
              <div className="flex items-center justify-between mb-1">
                <span className="text-[10px] text-text-muted uppercase tracking-wider">Typos</span>
                <span className="text-xs font-mono text-glow-amber">{(settings.errorRate * 100).toFixed(1)}%</span>
              </div>
              <input
                type="range" min="0" max="0.10" step="0.005"
                value={settings.errorRate}
                onChange={(e) => settings.setErrorRate(parseFloat(e.target.value))}
                className="w-full accent-glow-amber h-1"
              />
            </div>
          </div>

          {/* Advanced settings toggle */}
          <button
            onClick={settings.toggleAdvanced}
            className="flex items-center gap-1.5 text-[11px] text-text-muted hover:text-text-secondary transition-colors cursor-pointer self-start"
          >
            <svg
              width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"
              className={`transition-transform ${settings.advancedOpen ? "rotate-90" : ""}`}
            >
              <path d="M9 18l6-6-6-6" />
            </svg>
            Advanced Settings
          </button>

          {/* Advanced settings panel */}
          {settings.advancedOpen && (
            <div className="glass-subtle p-4 flex flex-col gap-4 animate-fade-in">
              {/* Behaviors section */}
              <div>
                <span className="text-[10px] text-text-muted uppercase tracking-wider">Behaviors</span>
                <div className="grid grid-cols-2 gap-2 mt-2">
                  <Toggle label="Thinking Pauses" enabled={settings.thinkingPausesEnabled} onChange={settings.setThinkingPauses} />
                  <Toggle label="Fatigue System" enabled={settings.fatigueEnabled} onChange={settings.setFatigue} />
                  <Toggle label="Burst Mode" enabled={settings.burstMode} onChange={settings.setBurstMode} />
                  <Toggle label="Key Rollover" enabled={settings.rolloverEnabled} onChange={settings.setRollover} />
                  <Toggle label="Micro-Corrections" enabled={settings.microCorrectionsEnabled} onChange={settings.setMicroCorrections} />
                  <Toggle label="Second Thoughts" enabled={settings.secondThoughtsEnabled} onChange={settings.setSecondThoughts} />
                  <Toggle label="Error Clustering" enabled={settings.errorClusteringEnabled} onChange={settings.setErrorClustering} />
                  <Toggle label="Punctuation Pauses" enabled={settings.punctuationPausesEnabled} onChange={settings.setPunctuationPauses} />
                </div>
              </div>

              {/* Error types section */}
              <div>
                <span className="text-[10px] text-text-muted uppercase tracking-wider">Error Types</span>
                <div className="grid grid-cols-3 gap-2 mt-2">
                  <Toggle label="Substitution" enabled={settings.substitutionEnabled} onChange={settings.setSubstitution} />
                  <Toggle label="Insertion" enabled={settings.insertionEnabled} onChange={settings.setInsertion} />
                  <Toggle label="Omission" enabled={settings.omissionEnabled} onChange={settings.setOmission} />
                  <Toggle label="Double Letter" enabled={settings.doubleLetterEnabled} onChange={settings.setDoubleLetter} />
                  <Toggle label="Transposition" enabled={settings.transpositionEnabled} onChange={settings.setTransposition} />
                </div>
              </div>

              {/* Corrections section */}
              <div>
                <span className="text-[10px] text-text-muted uppercase tracking-wider">Corrections</span>
                <div className="flex items-center gap-4 mt-2">
                  <Toggle label="Auto-Correct" enabled={settings.correctionEnabled} onChange={settings.setCorrectionEnabled} />
                  <div className="flex items-center gap-2">
                    <span className="text-[11px] text-text-secondary">Speed</span>
                    <select
                      value={settings.correctionSpeed}
                      onChange={(e) => settings.setCorrectionSpeed(e.target.value as CorrectionSpeed)}
                      className="bg-white/5 border border-white/10 rounded px-2 py-1 text-[11px] text-text-primary outline-none cursor-pointer"
                    >
                      <option value="Instant">Instant</option>
                      <option value="Quick">Quick</option>
                      <option value="Normal">Normal</option>
                      <option value="Slow">Slow</option>
                    </select>
                  </div>
                </div>
              </div>

              {/* Fine-tune sliders */}
              <div>
                <span className="text-[10px] text-text-muted uppercase tracking-wider">Fine-Tune</span>
                <div className="grid grid-cols-2 gap-x-4 gap-y-2 mt-2">
                  <MiniSlider
                    label="Speed Variation" value={settings.speedVariation}
                    min={0.05} max={0.35} step={0.01}
                    display={`${Math.round(settings.speedVariation * 100)}%`}
                    onChange={settings.setSpeedVariation}
                  />
                  <MiniSlider
                    label="Thinking Freq" value={settings.thinkingFrequency}
                    min={0.01} max={0.12} step={0.01}
                    display={`${Math.round(settings.thinkingFrequency * 100)}%`}
                    onChange={settings.setThinkingFrequency}
                  />
                  <MiniSlider
                    label="Micro-Correct %" value={settings.microCorrectionChance}
                    min={0.01} max={0.08} step={0.005}
                    display={`${(settings.microCorrectionChance * 100).toFixed(1)}%`}
                    onChange={settings.setMicroCorrectionChance}
                  />
                  <MiniSlider
                    label="2nd Thoughts %" value={settings.secondThoughtsChance}
                    min={0.005} max={0.06} step={0.005}
                    display={`${(settings.secondThoughtsChance * 100).toFixed(1)}%`}
                    onChange={settings.setSecondThoughtsChance}
                  />
                  <MiniSlider
                    label="Over-Backspace" value={settings.overBackspaceChance}
                    min={0} max={0.3} step={0.02}
                    display={`${Math.round(settings.overBackspaceChance * 100)}%`}
                    onChange={settings.setOverBackspaceChance}
                  />
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] text-text-secondary">Paragraph Pause</span>
                    <select
                      value={settings.paragraphPause}
                      onChange={(e) => settings.setParagraphPause(e.target.value as ParagraphPause)}
                      className="bg-white/5 border border-white/10 rounded px-2 py-0.5 text-[11px] text-text-primary outline-none cursor-pointer"
                    >
                      <option value="None">None</option>
                      <option value="Short">Short</option>
                      <option value="Normal">Normal</option>
                      <option value="Long">Long</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
          )}
        </>
      )}

      {/* Action buttons */}
      <div className="flex items-center gap-3">
        {isIdle ? (
          <>
            <button
              onClick={handleStart}
              disabled={!inputText.trim()}
              className="flex-1 py-2.5 rounded-lg font-semibold text-sm transition-all cursor-pointer disabled:opacity-30 disabled:cursor-not-allowed bg-glow-green/20 text-glow-green border border-glow-green/30 hover:bg-glow-green/30 hover:shadow-[0_0_20px_rgba(34,197,94,0.2)]"
            >
              Start Typing
            </button>
            <button
              onClick={handlePreview}
              disabled={!inputText.trim()}
              className="px-5 py-2.5 rounded-lg font-semibold text-sm transition-all cursor-pointer disabled:opacity-30 disabled:cursor-not-allowed bg-glow-purple/15 text-glow-purple border border-glow-purple/30 hover:bg-glow-purple/25"
            >
              Preview
            </button>
          </>
        ) : status === "running" ? (
          <>
            <button
              onClick={handlePause}
              className="flex-1 py-2.5 rounded-lg font-semibold text-sm transition-all cursor-pointer bg-glow-amber/20 text-glow-amber border border-glow-amber/30 hover:bg-glow-amber/30"
            >
              Pause
            </button>
            <button
              onClick={handleStop}
              className="px-6 py-2.5 rounded-lg font-semibold text-sm transition-all cursor-pointer bg-white/5 text-text-secondary border border-white/10 hover:bg-white/10 hover:text-glow-red"
            >
              Stop
            </button>
          </>
        ) : status === "paused" ? (
          <>
            <button
              onClick={handleResume}
              className="flex-1 py-2.5 rounded-lg font-semibold text-sm transition-all cursor-pointer bg-glow-green/20 text-glow-green border border-glow-green/30 hover:bg-glow-green/30"
            >
              Resume
            </button>
            <button
              onClick={handleStop}
              className="px-6 py-2.5 rounded-lg font-semibold text-sm transition-all cursor-pointer bg-white/5 text-text-secondary border border-white/10 hover:bg-white/10 hover:text-glow-red"
            >
              Stop
            </button>
          </>
        ) : null}
      </div>

      {/* Helper text */}
      {isIdle && (
        <p className="text-[11px] text-text-muted text-center">
          Start = 5s countdown then types into active app &middot; Preview = simulate in-app only
        </p>
      )}
    </div>
  );
}

function Toggle({ label, enabled, onChange }: { label: string; enabled: boolean; onChange: (v: boolean) => void }) {
  return (
    <button
      onClick={() => onChange(!enabled)}
      className="flex items-center gap-2 px-2 py-1.5 rounded-md transition-colors cursor-pointer hover:bg-white/5"
    >
      <div className={`w-7 h-4 rounded-full transition-colors relative shrink-0 ${enabled ? "bg-glow-cyan/30" : "bg-white/10"}`}>
        <div className={`absolute top-0.5 w-3 h-3 rounded-full transition-all ${enabled ? "left-3.5 bg-glow-cyan" : "left-0.5 bg-text-muted"}`} />
      </div>
      <span className="text-[11px] text-text-secondary">{label}</span>
    </button>
  );
}

function MiniSlider({ label, value, min, max, step, display, onChange }: {
  label: string; value: number; min: number; max: number; step: number; display: string; onChange: (v: number) => void;
}) {
  return (
    <div>
      <div className="flex items-center justify-between mb-0.5">
        <span className="text-[11px] text-text-secondary">{label}</span>
        <span className="text-[11px] font-mono text-text-muted">{display}</span>
      </div>
      <input
        type="range" min={min} max={max} step={step} value={value}
        onChange={(e) => onChange(parseFloat(e.target.value))}
        className="w-full accent-glow-cyan h-0.5"
      />
    </div>
  );
}
