import { useSettingsStore } from "../../stores/settings-store";

const presets = [
  { label: "Hunt & Peck", wpm: 28, range: "20-35" },
  { label: "Beginner", wpm: 42, range: "35-50" },
  { label: "Casual", wpm: 57, range: "50-65" },
  { label: "Average", wpm: 72, range: "65-80" },
  { label: "Skilled", wpm: 87, range: "80-95" },
  { label: "Professional", wpm: 102, range: "95-110" },
  { label: "Expert", wpm: 120, range: "110-130" },
];

export function SpeedSlider() {
  const { customWpm, setWpm } = useSettingsStore();

  const closestPreset = presets.reduce((prev, curr) =>
    Math.abs(curr.wpm - customWpm) < Math.abs(prev.wpm - customWpm) ? curr : prev
  );

  return (
    <div className="glass-subtle p-4">
      <div className="flex items-center justify-between mb-3">
        <span className="text-sm text-text-secondary">Typing Speed</span>
        <span className="text-sm font-mono text-glow-cyan">{customWpm} WPM</span>
      </div>
      <input
        type="range"
        min="20"
        max="130"
        step="1"
        value={customWpm}
        onChange={(e) => setWpm(parseInt(e.target.value))}
        className="w-full accent-glow-cyan"
      />
      <div className="flex justify-between mt-2">
        {presets.map((p) => (
          <button
            key={p.label}
            onClick={() => setWpm(p.wpm)}
            className={`text-[9px] px-1 py-0.5 rounded transition-colors cursor-pointer ${
              closestPreset.label === p.label
                ? "text-glow-cyan bg-glow-cyan/10"
                : "text-text-muted hover:text-text-secondary"
            }`}
          >
            {p.label.split(" ")[0]}
          </button>
        ))}
      </div>
    </div>
  );
}
