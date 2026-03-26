import { useEngineStore } from "../../stores/engine-store";
import { stopTyping } from "../../lib/tauri-commands";

export function CountdownOverlay() {
  const { status, countdown } = useEngineStore();

  if (status !== "countdown" || countdown <= 0) return null;

  const handleCancel = async () => {
    await stopTyping();
    useEngineStore.getState().setStatus("idle");
  };

  const circumference = 2 * Math.PI * 45; // ~283

  return (
    <div className="fixed inset-0 z-50 flex flex-col items-center justify-center bg-black/97 backdrop-blur-sm">
      <p className="text-text-muted text-sm mb-6">Switch to your target application</p>

      <div className="relative mb-4" key={countdown}>
        {/* SVG countdown ring */}
        <svg width="120" height="120" className="absolute inset-0 -rotate-90">
          {/* Background ring */}
          <circle
            cx="60" cy="60" r="45"
            fill="none"
            stroke="rgba(255,255,255,0.06)"
            strokeWidth="2"
          />
          {/* Animated ring */}
          <circle
            cx="60" cy="60" r="45"
            fill="none"
            stroke="rgba(52,211,153,0.4)"
            strokeWidth="2"
            strokeLinecap="round"
            strokeDasharray={circumference}
            strokeDashoffset="0"
            style={{ animation: "countdown-ring 1s linear forwards" }}
          />
        </svg>

        {/* Number */}
        <div
          className="w-[120px] h-[120px] flex items-center justify-center text-[80px] font-light text-white tabular-nums leading-none animate-count-pulse"
          style={{
            textShadow: "0 0 40px rgba(52,211,153,0.3), 0 0 80px rgba(52,211,153,0.1)",
          }}
        >
          {countdown}
        </div>
      </div>

      <p className="text-text-muted text-xs mb-10 font-mono">typing starts in {countdown}s</p>

      <button
        onClick={handleCancel}
        className="h-9 px-6 rounded-full text-[13px] text-text-muted border border-border hover:text-red hover:border-red/30 active:scale-95 cursor-pointer"
      >
        Cancel
      </button>
    </div>
  );
}
