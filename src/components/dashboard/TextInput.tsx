import { useState, useCallback, useEffect, useRef, useMemo } from "react";
import { useEngineStore } from "../../stores/engine-store";
import { useHumanizerStore } from "../../stores/humanizer-store";

const HUMANIZER_PHRASES = [
  "Restructuring syntax patterns",
  "Analyzing sentence entropy",
  "Injecting lexical variance",
  "Recalibrating tone signatures",
  "Dissolving predictive markers",
  "Reconstructing paragraph flow",
  "Fragmenting uniform cadence",
  "Embedding natural irregularities",
  "Neutralizing statistical fingerprints",
  "Diversifying vocabulary distribution",
  "Scrambling perplexity signals",
  "Resequencing clause boundaries",
  "Introducing burstiness variance",
  "Degrading pattern uniformity",
  "Simulating cognitive drift",
  "Randomizing transition structures",
  "Obscuring generative artifacts",
  "Modulating sentence complexity",
  "Disrupting token predictability",
  "Applying stylometric noise",
];

export function TextInput() {
  const { inputText, setInputText, totalChars } = useEngineStore();
  const humanizerStatus = useHumanizerStore((s) => s.status);
  const [dragging, setDragging] = useState(false);
  const [revealing, setRevealing] = useState(false);
  const prevStatus = useRef(humanizerStatus);

  const isHumanizing = humanizerStatus === "running";
  const [phraseIndex, setPhraseIndex] = useState(0);

  // Shuffle phrases once per humanize run
  const shuffledPhrases = useMemo(() => {
    if (!isHumanizing) return HUMANIZER_PHRASES;
    return [...HUMANIZER_PHRASES].sort(() => Math.random() - 0.5);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isHumanizing]);

  // Cycle through phrases while humanizing
  useEffect(() => {
    if (!isHumanizing) {
      setPhraseIndex(0);
      return;
    }
    const interval = setInterval(() => {
      setPhraseIndex((i) => (i + 1) % shuffledPhrases.length);
    }, 2200);
    return () => clearInterval(interval);
  }, [isHumanizing, shuffledPhrases]);

  // Trigger reveal animation when humanizer completes
  useEffect(() => {
    if (prevStatus.current === "running" && humanizerStatus === "done") {
      setRevealing(true);
      const timer = setTimeout(() => setRevealing(false), 700);
      return () => clearTimeout(timer);
    }
    prevStatus.current = humanizerStatus;
  }, [humanizerStatus]);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragging(false);
  }, []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragging(false);

    const file = e.dataTransfer.files?.[0];
    if (!file) return;

    // Accept .txt and other plain text files
    if (!file.name.endsWith(".txt") && !file.type.startsWith("text/")) return;

    const reader = new FileReader();
    reader.onload = (ev) => {
      const text = ev.target?.result;
      if (typeof text === "string") {
        setInputText(text);
      }
    };
    reader.readAsText(file);
  }, [setInputText]);

  return (
    <div className="flex-1 flex flex-col min-h-0 gap-2">
      <div className="flex items-center justify-between px-1">
        <span className="text-[10px] text-text-muted uppercase tracking-[0.15em] font-medium">Input</span>
        {inputText.length > 0 && (
          <span className="text-[11px] text-text-muted font-mono">
            {totalChars.toLocaleString()} chars
          </span>
        )}
      </div>

      <div
        className={`flex-1 rounded-xl bg-surface-raised overflow-hidden card-raised text-input-container relative transition-colors ${
          dragging ? "border-green/30" : ""
        } ${isHumanizing ? "humanizer-loading humanizer-shimmer" : ""}`}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
      >
        {/* Empty state overlay */}
        {!inputText && !isHumanizing && (
          <div className={`absolute inset-0 flex flex-col items-center justify-center pointer-events-none z-10 gap-3 transition-opacity ${
            dragging ? "opacity-100" : ""
          }`}>
            <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"
              className={dragging ? "text-green/30" : "text-white/[0.08]"}
              style={{ transition: "color 150ms ease" }}
            >
              <rect x="8" y="2" width="8" height="4" rx="1" ry="1" />
              <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
            </svg>
            <div className="flex flex-col items-center gap-1">
              <span className={`text-[13px] ${dragging ? "text-green/40" : "text-white/[0.12]"}`}
                style={{ transition: "color 150ms ease" }}
              >
                {dragging ? "Drop to load" : "Paste the text you want typed"}
              </span>
              {!dragging && (
                <span className="text-[11px] text-white/[0.06]">or drag a .txt file</span>
              )}
            </div>
          </div>
        )}

        {/* Humanizer loading overlay */}
        {isHumanizing && (
          <div className="absolute inset-0 flex flex-col items-center justify-center z-30 rounded-xl"
            style={{ background: "rgba(10, 10, 10, 0.7)", backdropFilter: "blur(2px)" }}
          >
            <div className="flex flex-col items-center gap-5">
              {/* Spinning ring */}
              <div className="relative w-10 h-10">
                <div className="absolute inset-0 rounded-full border-2 border-purple/10" />
                <div className="absolute inset-0 rounded-full border-2 border-transparent border-t-purple animate-spin" />
              </div>
              <div className="flex flex-col items-center gap-2">
                <span className="text-[14px] text-purple font-medium tracking-wide">
                  {shuffledPhrases[phraseIndex]}
                </span>
                <div className="flex items-center gap-1">
                  <span className="w-1 h-1 rounded-full bg-purple/40 animate-pulse" style={{ animationDelay: "0ms" }} />
                  <span className="w-1 h-1 rounded-full bg-purple/40 animate-pulse" style={{ animationDelay: "300ms" }} />
                  <span className="w-1 h-1 rounded-full bg-purple/40 animate-pulse" style={{ animationDelay: "600ms" }} />
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Drag overlay when text already present */}
        {inputText && dragging && (
          <div className="absolute inset-0 flex items-center justify-center z-30 bg-black/60 rounded-xl">
            <span className="text-[13px] text-green/50">Drop to replace</span>
          </div>
        )}

        <textarea
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          className={`w-full h-full bg-transparent text-white text-[13px] leading-6 p-4 resize-none outline-none
            placeholder:text-transparent font-mono rounded-xl relative z-20 ${
            revealing ? "humanizer-text-reveal" : ""
          }`}
          spellCheck={false}
          readOnly={isHumanizing}
        />
      </div>
    </div>
  );
}
