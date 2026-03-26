import { useSettingsStore } from "../../stores/settings-store";
import { personalities } from "../../lib/personalities";
import type { CorrectionSpeed, ParagraphPause, ThinkingPreset } from "../../stores/settings-store";

export function SettingsPanel() {
  const settings = useSettingsStore();

  const applyPersonality = (p: (typeof personalities)[0]) => {
    settings.setPersonality(p.id);
    settings.setWpm(p.wpm);
    settings.setErrorRate(p.errorRate);
    settings.setSpeedVariation(p.speedVariation);
    settings.setBurstMode(p.burstMode);
    settings.setRollover(p.rolloverEnabled);
    settings.setFatigue(p.fatigueResistance < 0.5);
    settings.setThinkingFrequency(p.thinkingFrequency);
  };

  return (
    <div className="h-full overflow-y-auto">
      <div className="p-4 flex flex-col">
        {/* Style / Personality */}
        <section>
          <SectionHeader>Style</SectionHeader>
          <div className="flex flex-col mt-2">
            {personalities.map((p) => {
              const active = settings.activePersonality === p.id;
              return (
                <button
                  key={p.id}
                  onClick={() => applyPersonality(p)}
                  className={`relative flex items-center gap-2 px-2 py-[5px] rounded-lg text-left cursor-pointer
                    ${active ? "bg-white/[0.07]" : "hover:bg-white/[0.04]"}`}
                >
                  {/* Left accent bar */}
                  <div
                    className="absolute left-0 top-1/2 -translate-y-1/2 w-[2px] rounded-full transition-all duration-200"
                    style={{
                      height: active ? 16 : 0,
                      background: active ? "#34d399" : "rgba(255,255,255,0.2)",
                      opacity: active ? 1 : 0,
                    }}
                  />
                  <span
                    className="text-[14px] w-5 text-center"
                    style={active ? { filter: "drop-shadow(0 0 4px rgba(52,211,153,0.6))" } : undefined}
                  >
                    {p.icon}
                  </span>
                  <span className={`text-[11px] flex-1 truncate ${active ? "text-white" : "text-text-secondary"}`}>
                    {p.name}
                  </span>
                  <span className="text-[10px] text-text-muted font-mono">{p.wpm}</span>
                </button>
              );
            })}
          </div>
        </section>

        <div className="section-divider mt-4 mb-4" />

        {/* Speed */}
        <section>
          <SectionHeader>Speed</SectionHeader>
          <div className="flex flex-col gap-2 mt-2">
            <Slider label="WPM" value={settings.customWpm} min={15} max={150} step={1}
              display={String(settings.customWpm)} onChange={settings.setWpm}
              hint="Base typing speed" />
            <Slider label="Typo rate" value={settings.errorRate} min={0} max={0.10} step={0.005}
              display={`${(settings.errorRate * 100).toFixed(1)}%`} onChange={settings.setErrorRate}
              hint="Chance of a typo per keystroke" />
            <Slider label="Variation" value={settings.speedVariation} min={0.05} max={0.35} step={0.01}
              display={`${Math.round(settings.speedVariation * 100)}%`} onChange={settings.setSpeedVariation}
              hint="Speed randomness between keys" />
          </div>
        </section>

        <div className="section-divider mt-4 mb-4" />

        {/* Behaviors */}
        <section>
          <SectionHeader>Behaviors</SectionHeader>
          <div className="flex flex-col mt-1">
            <Toggle label="Thinking pauses" enabled={settings.thinkingPausesEnabled} onChange={settings.setThinkingPauses}
              hint="Pauses as if deciding what to type" />
            <Toggle label="Fatigue" enabled={settings.fatigueEnabled} onChange={settings.setFatigue}
              hint="Slows down and makes more errors over time" />
            <Toggle label="Burst typing" enabled={settings.burstMode} onChange={settings.setBurstMode}
              hint="Fast bursts with short pauses between" />
            <Toggle label="Key rollover" enabled={settings.rolloverEnabled} onChange={settings.setRollover}
              hint="Next key pressed before current one releases" />
            <Toggle label="Micro-corrections" enabled={settings.microCorrectionsEnabled} onChange={settings.setMicroCorrections}
              hint="Backspace correct chars as if second-guessing" />
            <Toggle label="Second thoughts" enabled={settings.secondThoughtsEnabled} onChange={settings.setSecondThoughts}
              hint="Delete last few words and rephrase with synonyms" />
            {settings.secondThoughtsEnabled && (
              <div className="pl-1 py-1">
                <Slider label="Synonym swap" value={settings.synonymChance} min={0} max={1} step={0.05}
                  display={`${Math.round(settings.synonymChance * 100)}%`} onChange={settings.setSynonymChance}
                  hint="Chance each retyped word becomes a synonym" />
              </div>
            )}
            <Toggle label="Word substitution" enabled={settings.wordSubstitutionEnabled} onChange={settings.setWordSubstitution}
              hint="Start a wrong synonym, catch it, fix it" />
            <Toggle label="Unfamiliar slowdown" enabled={settings.unfamiliarWordSlowdown} onChange={settings.setUnfamiliarWordSlowdown}
              hint="Types slower on unusual words" />
            <Toggle label="Error clustering" enabled={settings.errorClusteringEnabled} onChange={settings.setErrorClustering}
              hint="One mistake makes the next more likely" />
          </div>
        </section>

        <div className="section-divider mt-4 mb-4" />

        {/* Hesitation */}
        <section>
          <SectionHeader>Hesitation</SectionHeader>
          <div className="flex flex-col mt-1">
            <Toggle label="Random backspace" enabled={settings.hesitationBackspaceEnabled} onChange={settings.setHesitationBackspace}
              hint="Delete and retype correct chars for no reason" />
            <Toggle label="Mid-word freeze" enabled={settings.midWordPauseEnabled} onChange={settings.setMidWordPause}
              hint="Pause mid-word as if unsure of spelling" />
            <Toggle label="Sentence restart" enabled={settings.sentenceRestartEnabled} onChange={settings.setSentenceRestart}
              hint="Delete a whole sentence and retype it" />
          </div>
        </section>

        <div className="section-divider mt-4 mb-4" />

        {/* Mistakes */}
        <section>
          <SectionHeader>Mistakes</SectionHeader>
          <div className="flex flex-col mt-1">
            <Toggle label="Wrong key" enabled={settings.substitutionEnabled} onChange={settings.setSubstitution}
              hint="Hits a nearby key instead" />
            <Toggle label="Extra key" enabled={settings.insertionEnabled} onChange={settings.setInsertion}
              hint="Inserts an extra character" />
            <Toggle label="Missed key" enabled={settings.omissionEnabled} onChange={settings.setOmission}
              hint="Skips a character entirely" />
            <Toggle label="Double press" enabled={settings.doubleLetterEnabled} onChange={settings.setDoubleLetter}
              hint="Same key pressed twice" />
            <Toggle label="Swapped keys" enabled={settings.transpositionEnabled} onChange={settings.setTransposition}
              hint="Two characters typed in wrong order" />
            <Toggle label="Wrong caps" enabled={settings.wrongCapsEnabled} onChange={settings.setWrongCaps}
              hint="Wrong uppercase/lowercase" />
          </div>
        </section>

        <div className="section-divider mt-4 mb-4" />

        {/* Corrections */}
        <section>
          <SectionHeader>Corrections</SectionHeader>
          <div className="flex flex-col mt-1">
            <Toggle label="Auto-correct" enabled={settings.correctionEnabled} onChange={settings.setCorrectionEnabled}
              hint="Notice and fix typos after making them" />
            <Toggle label="Punctuation pauses" enabled={settings.punctuationPausesEnabled} onChange={settings.setPunctuationPauses}
              hint="Brief pause after periods, commas, etc." />
            <div className="flex flex-col gap-1 mt-1">
              <SelectRow label="Correction speed" value={settings.correctionSpeed}
                onChange={(v) => settings.setCorrectionSpeed(v as CorrectionSpeed)}
                options={["Instant", "Quick", "Normal", "Slow", "VerySlow"]}
                hint="How fast typos get noticed and fixed" />
              <SelectRow label="Paragraph pause" value={settings.paragraphPause}
                onChange={(v) => settings.setParagraphPause(v as ParagraphPause)}
                options={["None", "Brief", "Short", "Normal", "Long", "VeryLong", "ExtendedBreak"]}
                hint="Pause length between paragraphs" />
              <SelectRow label="Thinking duration" value={settings.thinkingPreset}
                onChange={(v) => settings.setThinkingPreset(v as ThinkingPreset)}
                options={["Brief", "Short", "Normal", "Medium", "Long", "VeryLong", "ExtremelyLong"]}
                hint="How long thinking pauses last" />
            </div>
          </div>
        </section>

        <div className="section-divider mt-4 mb-4" />

        {/* Session */}
        <section>
          <SectionHeader>Session</SectionHeader>
          <div className="flex flex-col mt-1">
            <Toggle label="Auto-pause breaks" enabled={settings.autoPauseEnabled} onChange={settings.setAutoPause}
              hint="Periodic breaks during long sessions" />
            {settings.autoPauseEnabled && (
              <div className="pl-1 py-1 flex flex-col gap-2">
                <Slider label="Break every" value={settings.autoPauseInterval} min={5} max={60} step={5}
                  display={`${settings.autoPauseInterval}m`} onChange={settings.setAutoPauseInterval}
                  hint="Minutes between breaks" />
                <Slider label="Break duration" value={settings.autoPauseDuration} min={1} max={15} step={1}
                  display={`${settings.autoPauseDuration}m`} onChange={settings.setAutoPauseDuration}
                  hint="Length of each break" />
              </div>
            )}
          </div>
        </section>
      </div>
    </div>
  );
}

function SectionHeader({ children }: { children: React.ReactNode }) {
  return (
    <div className="text-[10px] uppercase tracking-[0.15em] font-medium" style={{ color: "#71717a" }}>
      {children}
    </div>
  );
}

function Toggle({ label, enabled, onChange, hint }: {
  label: string; enabled: boolean; onChange: (v: boolean) => void; hint?: string;
}) {
  return (
    <button
      onClick={() => onChange(!enabled)}
      className="group relative flex items-center justify-between py-[6px] cursor-pointer hover:bg-white/[0.03] rounded px-1 w-full text-left"
    >
      <span className="text-[12px] text-text-secondary">{label}</span>
      <div
        className={`relative shrink-0 rounded-full transition-colors duration-150 ${enabled ? "bg-green" : "bg-white/[0.12]"}`}
        style={{
          width: 28,
          height: 16,
          boxShadow: enabled ? "inset 0 1px 3px rgba(0,0,0,0.2), 0 0 8px rgba(52,211,153,0.15)" : undefined,
        }}
      >
        <div
          className="absolute rounded-full bg-white"
          style={{
            width: 12,
            height: 12,
            top: 2,
            left: enabled ? 14 : 2,
            boxShadow: "0 1px 2px rgba(0,0,0,0.4)",
            transition: "left 200ms cubic-bezier(0.34, 1.56, 0.64, 1)",
          }}
        />
      </div>
      {hint && <Tooltip text={hint} />}
    </button>
  );
}

function SelectRow({ label, value, onChange, options, hint }: {
  label: string; value: string; onChange: (v: string) => void; options: string[]; hint?: string;
}) {
  const idx = options.indexOf(value);
  const prev = () => { if (idx > 0) onChange(options[idx - 1]); };
  const next = () => { if (idx < options.length - 1) onChange(options[idx + 1]); };

  return (
    <div className="group relative flex items-center justify-between py-[5px] px-1">
      <span className="text-[12px] text-text-secondary">{label}</span>
      <div className="flex items-center gap-1">
        <button
          onClick={prev}
          className={`w-5 h-5 flex items-center justify-center rounded cursor-pointer ${
            idx > 0 ? "text-text-secondary hover:text-white hover:bg-white/[0.06]" : "text-white/[0.08] cursor-default"
          }`}
          disabled={idx <= 0}
        >
          <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
            <polyline points="15 18 9 12 15 6" />
          </svg>
        </button>
        <span className="text-[11px] text-text-secondary font-mono min-w-[60px] text-center select-none">
          {value}
        </span>
        <button
          onClick={next}
          className={`w-5 h-5 flex items-center justify-center rounded cursor-pointer ${
            idx < options.length - 1 ? "text-text-secondary hover:text-white hover:bg-white/[0.06]" : "text-white/[0.08] cursor-default"
          }`}
          disabled={idx >= options.length - 1}
        >
          <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
            <polyline points="9 18 15 12 9 6" />
          </svg>
        </button>
      </div>
      {hint && <Tooltip text={hint} />}
    </div>
  );
}

function Slider({ label, value, min, max, step, display, onChange, hint }: {
  label: string; value: number; min: number; max: number; step: number;
  display: string; onChange: (v: number) => void; hint?: string;
}) {
  const pct = ((value - min) / (max - min)) * 100;

  return (
    <div className="group relative">
      <div className="flex items-center justify-between mb-1">
        <span className="text-[11px] text-text-secondary">{label}</span>
        <span className="text-[11px] text-white font-mono">{display}</span>
      </div>
      <input
        type="range" min={min} max={max} step={step} value={value}
        onChange={(e) => onChange(parseFloat(e.target.value))}
        className="w-full"
        style={{
          background: `linear-gradient(to right, rgba(52,211,153,0.3) 0%, rgba(52,211,153,0.3) ${pct}%, rgba(255,255,255,0.08) ${pct}%, rgba(255,255,255,0.08) 100%)`,
          borderRadius: "1px",
        }}
      />
      {hint && <Tooltip text={hint} />}
    </div>
  );
}

function Tooltip({ text }: { text: string }) {
  return (
    <div className="absolute left-0 right-0 bottom-full mb-1.5 z-50
      opacity-0 group-hover:opacity-100 pointer-events-none
      transition-opacity duration-150 delay-300">
      <div className="bg-[#1c1c1c] border border-white/[0.08] rounded-md px-2.5 py-1.5 shadow-lg
        text-[10px] text-text-secondary leading-relaxed w-max max-w-[200px]">
        {text}
      </div>
    </div>
  );
}
