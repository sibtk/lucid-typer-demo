import { personalities } from "../../lib/personalities";
import { useSettingsStore } from "../../stores/settings-store";

export function Sidebar() {
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
    <div className="w-14 h-full flex flex-col items-center pt-4 gap-1 border-r border-border">
      {personalities.map((p) => {
        const active = settings.activePersonality === p.id;
        return (
          <div key={p.id} className="relative group">
            <button
              onClick={() => applyPersonality(p)}
              className={`w-9 h-9 flex items-center justify-center rounded-lg transition-all duration-150 cursor-pointer text-[16px]
                ${active ? "bg-green/10" : "hover:bg-white/[0.04]"}`}
              style={
                active
                  ? { boxShadow: "0 0 12px rgba(52,211,153,0.3), inset 0 0 8px rgba(52,211,153,0.1)" }
                  : undefined
              }
            >
              {p.icon}
            </button>
            <div
              className="absolute left-full ml-2 top-1/2 -translate-y-1/2 px-2.5 py-1.5 rounded-md
                bg-surface-raised border border-border text-[11px] text-text-secondary whitespace-nowrap
                opacity-0 pointer-events-none group-hover:opacity-100 transition-opacity duration-150 z-50"
            >
              {p.name}
              <span className="text-text-muted font-mono ml-1.5">{p.wpm}</span>
            </div>
          </div>
        );
      })}
    </div>
  );
}
