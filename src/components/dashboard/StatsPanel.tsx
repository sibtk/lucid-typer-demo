import { useEngineStore } from "../../stores/engine-store";

export function StatsPanel() {
  const { currentWpm, errorCount, typedChars, totalChars, elapsedMs, progress } = useEngineStore();

  const minutes = Math.floor(elapsedMs / 60000);
  const seconds = Math.floor((elapsedMs % 60000) / 1000);
  const timeStr = `${minutes}:${seconds.toString().padStart(2, "0")}`;
  const errorRate = typedChars > 0 ? ((errorCount / typedChars) * 100).toFixed(1) : "0.0";
  const progressPct = Math.round(progress * 100);

  return (
    <div className="flex items-center gap-8 text-[12px]">
      <Stat label="WPM" value={String(currentWpm)} />
      <Stat label="Errors" value={`${errorRate}%`} />
      <Stat label="Time" value={timeStr} />
      <Stat label="Progress" value={`${progressPct}%`} />
      <div className="flex-1 h-[3px] rounded-full bg-white/[0.04] overflow-hidden">
        <div
          className="h-full rounded-full bg-white/30 transition-all duration-300"
          style={{ width: `${progressPct}%` }}
        />
      </div>
      <span className="text-text-muted font-mono text-[11px]">{typedChars}/{totalChars}</span>
    </div>
  );
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center gap-2">
      <span className="text-text-muted">{label}</span>
      <span className="text-text-primary font-mono font-medium">{value}</span>
    </div>
  );
}
