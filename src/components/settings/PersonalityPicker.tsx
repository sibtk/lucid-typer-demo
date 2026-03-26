import { personalities } from "../../lib/personalities";
import { useSettingsStore } from "../../stores/settings-store";

const glowRing: Record<string, string> = {
  cyan: "ring-glow-cyan/50 shadow-[0_0_12px_rgba(0,212,255,0.2)]",
  purple: "ring-glow-purple/50 shadow-[0_0_12px_rgba(168,85,247,0.2)]",
  green: "ring-glow-green/50 shadow-[0_0_12px_rgba(34,197,94,0.2)]",
  amber: "ring-glow-amber/50 shadow-[0_0_12px_rgba(245,158,11,0.2)]",
  pink: "ring-glow-pink/50 shadow-[0_0_12px_rgba(236,72,153,0.2)]",
  red: "ring-glow-red/50 shadow-[0_0_12px_rgba(239,68,68,0.2)]",
};

export function PersonalityPicker() {
  const { activePersonality, setPersonality, setWpm, setErrorRate } = useSettingsStore();

  return (
    <div className="grid grid-cols-2 gap-2">
      {personalities.map((p) => {
        const isActive = activePersonality === p.id;
        return (
          <div
            key={p.id}
            onClick={() => {
              setPersonality(p.id);
              setWpm(p.wpm);
              setErrorRate(p.errorRate);
            }}
            className={`
              glass-subtle p-3 cursor-pointer transition-all duration-200 hover:scale-[1.02]
              ${isActive ? `ring-1 ${glowRing[p.glowColor]}` : "hover:bg-white/5"}
            `}
          >
            <div className="flex items-center gap-2 mb-1">
              <span className="text-lg">{p.icon}</span>
              <span className="text-sm font-medium text-text-primary">{p.name}</span>
            </div>
            <p className="text-[11px] text-text-muted leading-snug">{p.description}</p>
            <div className="flex gap-3 mt-2 text-[10px] text-text-muted">
              <span>{p.wpm} WPM</span>
              <span>{(p.errorRate * 100).toFixed(1)}% err</span>
            </div>
          </div>
        );
      })}
    </div>
  );
}
