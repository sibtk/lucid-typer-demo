import { create } from "zustand";

export type EngineStatus = "idle" | "running" | "paused" | "completed" | "countdown";

export interface CharState {
  char: string;
  state: "correct" | "error" | "correcting" | "pending";
}

export interface WaveformPoint {
  timestamp: number;
  wpm: number;
}

interface EngineStore {
  status: EngineStatus;
  countdown: number;
  progress: number;
  currentWpm: number;
  targetWpm: number;
  errorCount: number;
  totalChars: number;
  typedChars: number;
  elapsedMs: number;
  waveformData: WaveformPoint[];
  previewChars: CharState[];
  inputText: string;
  setInputText: (text: string) => void;
  setStatus: (status: EngineStatus) => void;
  setCountdown: (n: number) => void;
  updateProgress: (event: {
    position: number;
    total: number;
    current_wpm: number;
    error_count: number;
    elapsed_ms: number;
  }) => void;
  addWaveformPoint: (point: WaveformPoint) => void;
  updateCharState: (index: number, state: CharState["state"]) => void;
  reset: () => void;
}

export const useEngineStore = create<EngineStore>((set) => ({
  status: "idle",
  countdown: 0,
  progress: 0,
  currentWpm: 0,
  targetWpm: 65,
  errorCount: 0,
  totalChars: 0,
  typedChars: 0,
  elapsedMs: 0,
  waveformData: [],
  previewChars: [],
  inputText: "",

  setInputText: (text) => {
    const chars: CharState[] = text.split("").map((char) => ({
      char,
      state: "pending" as const,
    }));
    set({ inputText: text, previewChars: chars, totalChars: text.length });
  },

  setStatus: (status) => set({ status }),
  setCountdown: (n) => set({ countdown: n }),

  updateProgress: (event) => {
    const progress = event.total > 0 ? event.position / event.total : 0;
    set({
      typedChars: event.position,
      totalChars: event.total,
      currentWpm: Math.round(event.current_wpm),
      errorCount: event.error_count,
      elapsedMs: event.elapsed_ms,
      progress,
    });
  },

  addWaveformPoint: (point) =>
    set((state) => ({
      waveformData: [...state.waveformData.slice(-200), point],
    })),

  updateCharState: (index, charState) =>
    set((state) => {
      const newChars = [...state.previewChars];
      if (newChars[index]) {
        newChars[index] = { ...newChars[index], state: charState };
      }
      return { previewChars: newChars };
    }),

  reset: () =>
    set({
      status: "idle",
      countdown: 0,
      progress: 0,
      currentWpm: 0,
      errorCount: 0,
      typedChars: 0,
      elapsedMs: 0,
      waveformData: [],
    }),
}));
