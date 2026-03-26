import { useEngineStore } from "../../stores/engine-store";

export function TypingView() {
  const { inputText, typedChars, progress, status } = useEngineStore();
  const progressPct = Math.round(progress * 100);

  return (
    <div className="flex-1 flex flex-col min-h-0 rounded-xl bg-surface-raised overflow-hidden card-raised">
      <div className="flex-1 overflow-y-auto px-5 py-4">
        <pre className="text-[13px] leading-6 font-mono whitespace-pre-wrap break-words">
          {inputText.split("").map((char, i) => (
            <span
              key={i}
              className={
                i < typedChars
                  ? "text-white/90"
                  : i === typedChars && status === "running"
                  ? "text-green bg-green/15 rounded-sm"
                  : "text-white/20"
              }
              style={
                i === typedChars && status === "running"
                  ? { textShadow: "0 0 8px rgba(52,211,153,0.5)" }
                  : undefined
              }
            >
              {char}
            </span>
          ))}
        </pre>
      </div>

      <div className="shrink-0 px-5 pb-3">
        <div className="flex items-center gap-3">
          <div className="flex-1 h-[3px] rounded-full bg-white/[0.06] overflow-hidden">
            <div
              className="h-full rounded-full transition-all duration-300 relative"
              style={{
                width: `${progressPct}%`,
                background: "linear-gradient(90deg, rgba(52,211,153,0.4), rgba(52,211,153,0.7))",
                boxShadow: "0 0 8px rgba(52,211,153,0.3), 0 0 2px rgba(52,211,153,0.5)",
              }}
            >
              {/* Luminous leading edge */}
              <div
                className="absolute right-0 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full"
                style={{
                  background: "rgba(52,211,153,0.8)",
                  boxShadow: "0 0 6px rgba(52,211,153,0.6), 0 0 12px rgba(52,211,153,0.3)",
                }}
              />
            </div>
          </div>
          <span className="text-[11px] text-text-muted font-mono">{progressPct}%</span>
        </div>
      </div>
    </div>
  );
}
