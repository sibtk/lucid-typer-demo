import { useEngineStore } from "../../stores/engine-store";

export function PreviewPane() {
  const { previewChars, typedChars, status, inputText } = useEngineStore();

  if (!inputText) {
    return (
      <div className="glass p-4 flex items-center justify-center h-full">
        <span className="text-text-muted text-sm">Preview will appear when you start typing</span>
      </div>
    );
  }

  return (
    <div className="glass p-4 flex flex-col h-full">
      <span className="text-[10px] text-text-muted uppercase tracking-wider mb-2">Live Preview</span>
      <div className="flex gap-4 flex-1 overflow-hidden">
        {/* Original text (left) */}
        <div className="flex-1 overflow-y-auto pr-2">
          <div className="text-[10px] text-text-muted mb-1 uppercase tracking-wider">Original</div>
          <pre className="text-sm font-mono leading-relaxed whitespace-pre-wrap text-text-secondary">
            {previewChars.map((c, i) => (
              <span
                key={i}
                className={
                  i < typedChars
                    ? "text-text-muted/40"
                    : i === typedChars
                    ? "text-text-primary underline underline-offset-4 decoration-glow-cyan"
                    : ""
                }
              >
                {c.char}
              </span>
            ))}
          </pre>
        </div>

        {/* Divider */}
        <div className="w-px bg-white/5" />

        {/* Simulated output (right) */}
        <div className="flex-1 overflow-y-auto pr-2">
          <div className="text-[10px] text-text-muted mb-1 uppercase tracking-wider">Simulated</div>
          <pre className="text-sm font-mono leading-relaxed whitespace-pre-wrap">
            {previewChars.slice(0, typedChars).map((c, i) => (
              <span
                key={i}
                className={
                  c.state === "error"
                    ? "text-glow-red bg-glow-red/10 animate-error-flash"
                    : c.state === "correcting"
                    ? "text-glow-amber"
                    : "text-text-primary"
                }
              >
                {c.char}
              </span>
            ))}
            {status === "running" && (
              <span className="inline-block w-0.5 h-4 bg-glow-cyan animate-cursor-blink align-text-bottom ml-px" />
            )}
          </pre>
        </div>
      </div>
    </div>
  );
}
