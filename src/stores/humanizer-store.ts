import { create } from "zustand";
import { humanizeText, type PipelineEvent } from "../lib/humanizer-api";

type HumanizerStatus = "idle" | "running" | "done" | "error";
type HumanizerMode = "light" | "balanced" | "aggressive";

interface HumanizerStore {
  status: HumanizerStatus;
  mode: HumanizerMode;
  score: number | null;
  statusMessage: string;
  outputText: string;
  error: string | null;
  abortController: AbortController | null;

  setMode: (mode: HumanizerMode) => void;
  start: (text: string, onComplete: (text: string) => void) => void;
  reset: () => void;
}

export const useHumanizerStore = create<HumanizerStore>((set, get) => ({
  status: "idle",
  mode: "balanced",
  score: null,
  statusMessage: "",
  outputText: "",
  error: null,
  abortController: null,

  setMode: (mode) => set({ mode }),

  start: (text, onComplete) => {
    const ac = new AbortController();
    set({
      status: "running",
      score: null,
      statusMessage: "Starting humanizer...",
      outputText: "",
      error: null,
      abortController: ac,
    });

    humanizeText(
      text,
      get().mode,
      (event: PipelineEvent) => {
        switch (event.type) {
          case "pipeline_start":
            set({ statusMessage: "Starting pipeline..." });
            break;
          case "pass_start":
            if (event.passType === "full_rewrite") {
              set({ statusMessage: "Rewriting text..." });
            } else if (event.passType === "detection") {
              set({ statusMessage: `Checking AI score...` });
            } else if (event.passType === "targeted_rewrite") {
              set({ statusMessage: "Rewriting flagged sentences..." });
            } else if (event.passType === "polish") {
              set({ statusMessage: "Polishing..." });
            } else {
              set({ statusMessage: `Pass ${event.passNumber || ""}...` });
            }
            break;
          case "pass_complete":
            if (event.text) set({ outputText: event.text });
            if (event.score != null) set({ score: Math.round((1 - event.score) * 100) });
            break;
          case "detection_result":
            if (event.overallScore != null) {
              set({ score: Math.round((1 - event.overallScore) * 100) });
            }
            break;
          case "pipeline_complete": {
            const finalText = event.humanizedText || event.text || get().outputText;
            const finalScore = event.finalScore != null
              ? Math.round((1 - event.finalScore) * 100)
              : get().score;
            set({
              status: "done",
              outputText: finalText,
              score: finalScore,
              statusMessage: "",
              abortController: null,
            });
            if (finalText) onComplete(finalText);
            break;
          }
          case "error":
            set({
              status: "error",
              error: event.message || "Humanizer failed",
              statusMessage: "",
              abortController: null,
            });
            break;
        }
      },
      ac.signal,
    ).catch((err) => {
      if (err.name === "AbortError") return;
      set({
        status: "error",
        error: err.message || "Humanizer failed",
        statusMessage: "",
        abortController: null,
      });
    });
  },

  reset: () => {
    const ac = get().abortController;
    if (ac) ac.abort();
    set({
      status: "idle",
      score: null,
      statusMessage: "",
      outputText: "",
      error: null,
      abortController: null,
    });
  },
}));
