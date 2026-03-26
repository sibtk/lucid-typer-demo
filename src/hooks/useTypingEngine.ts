import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useEngineStore } from "../stores/engine-store";
import { useToastStore } from "../stores/toast-store";

export function useTypingEngine() {
  const { addWaveformPoint, updateProgress, updateCharState, setStatus, setCountdown } = useEngineStore();

  useEffect(() => {
    let cancelled = false;
    const unlisten: Array<() => void> = [];

    const setup = async () => {
      const listeners = await Promise.all([
        listen<any>("typing-progress", (event) => {
          const p = event.payload;
          updateProgress({
            position: p.position,
            total: p.total,
            current_wpm: p.current_wpm,
            error_count: p.error_count,
            elapsed_ms: p.elapsed_ms,
          });
          addWaveformPoint({
            timestamp: p.elapsed_ms,
            wpm: p.current_wpm,
          });
        }),

        listen<any>("typing-char", (event) => {
          const p = event.payload;
          if (p.char_typed !== "\x08") {
            updateCharState(p.position, p.is_correct ? "correct" : "error");
          }
        }),

        listen<any>("typing-error", (event) => {
          const p = event.payload;
          updateCharState(p.position, "error");
        }),

        listen<any>("typing-correction", (event) => {
          const p = event.payload;
          updateCharState(p.position, "correct");
        }),

        listen<any>("typing-state", (event) => {
          const p = event.payload;
          if (p.state === "completed") {
            setStatus("completed");
            useToastStore.getState().addToast("Typing complete", "success");
          } else if (p.state === "cancelled") {
            setStatus("idle");
          } else if (p.state === "running") {
            setStatus("running");
          } else if (p.state === "countdown") {
            setStatus("countdown");
          }
        }),

        listen<any>("typing-countdown", (event) => {
          const p = event.payload;
          setCountdown(p.seconds_left);
          if (p.seconds_left === 0) {
            setStatus("running");
          }
        }),

        listen<any>("typing-synonym", (event) => {
          const p = event.payload;
          useToastStore.getState().addToast(
            `${p.original} → ${p.replacement}`,
            "info"
          );
        }),
      ]);

      if (cancelled) {
        listeners.forEach((fn) => fn());
      } else {
        unlisten.push(...listeners);
      }
    };

    setup();
    return () => {
      cancelled = true;
      unlisten.forEach((fn) => fn());
    };
  }, []);
}
