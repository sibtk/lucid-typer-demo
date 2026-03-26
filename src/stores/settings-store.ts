import { create } from "zustand";
import { persist } from "zustand/middleware";

export type InteractionMode = "main" | "tray" | "overlay";
export type CorrectionSpeed = "Instant" | "Quick" | "Normal" | "Slow" | "VerySlow";
export type ParagraphPause = "None" | "Brief" | "Short" | "Normal" | "Long" | "VeryLong" | "ExtendedBreak";
export type ThinkingPreset = "Brief" | "Short" | "Normal" | "Medium" | "Long" | "VeryLong" | "ExtremelyLong";

interface SettingsStore {
  activePersonality: string;
  customWpm: number;
  interactionMode: InteractionMode;
  advancedOpen: boolean;
  settingsPanelOpen: boolean;
  errorRate: number;
  speedVariation: number;

  // Behavior toggles
  thinkingPausesEnabled: boolean;
  thinkingFrequency: number;
  thinkingPreset: ThinkingPreset;
  fatigueEnabled: boolean;
  burstMode: boolean;
  rolloverEnabled: boolean;
  microCorrectionsEnabled: boolean;
  microCorrectionChance: number;
  secondThoughtsEnabled: boolean;
  secondThoughtsChance: number;
  synonymChance: number;
  errorClusteringEnabled: boolean;

  // New behavior toggles
  wordSubstitutionEnabled: boolean;
  wordSubstitutionChance: number;
  hesitationBackspaceEnabled: boolean;
  hesitationBackspaceChance: number;
  midWordPauseEnabled: boolean;
  midWordPauseChance: number;
  sentenceRestartEnabled: boolean;
  sentenceRestartChance: number;
  unfamiliarWordSlowdown: boolean;
  autoPauseEnabled: boolean;
  autoPauseInterval: number;
  autoPauseDuration: number;

  // Error type toggles
  substitutionEnabled: boolean;
  insertionEnabled: boolean;
  omissionEnabled: boolean;
  doubleLetterEnabled: boolean;
  transpositionEnabled: boolean;
  wrongCapsEnabled: boolean;

  // Correction settings
  correctionEnabled: boolean;
  correctionSpeed: CorrectionSpeed;
  overBackspaceChance: number;

  // Pause settings
  punctuationPausesEnabled: boolean;
  paragraphPause: ParagraphPause;

  // Per-punctuation pause overrides (ms ranges)
  periodPause: [number, number];
  commaPause: [number, number];
  questionPause: [number, number];
  exclamationPause: [number, number];
  colonPause: [number, number];
  semicolonPause: [number, number];

  // Actions
  setPersonality: (id: string) => void;
  setWpm: (wpm: number) => void;
  toggleAdvanced: () => void;
  setErrorRate: (rate: number) => void;
  setSpeedVariation: (v: number) => void;
  setThinkingPauses: (enabled: boolean) => void;
  setThinkingFrequency: (f: number) => void;
  setThinkingPreset: (p: ThinkingPreset) => void;
  setFatigue: (enabled: boolean) => void;
  setBurstMode: (enabled: boolean) => void;
  setRollover: (enabled: boolean) => void;
  setMicroCorrections: (enabled: boolean) => void;
  setMicroCorrectionChance: (c: number) => void;
  setSecondThoughts: (enabled: boolean) => void;
  setSecondThoughtsChance: (c: number) => void;
  setSynonymChance: (c: number) => void;
  setErrorClustering: (enabled: boolean) => void;
  setWordSubstitution: (enabled: boolean) => void;
  setWordSubstitutionChance: (c: number) => void;
  setHesitationBackspace: (enabled: boolean) => void;
  setHesitationBackspaceChance: (c: number) => void;
  setMidWordPause: (enabled: boolean) => void;
  setMidWordPauseChance: (c: number) => void;
  setSentenceRestart: (enabled: boolean) => void;
  setSentenceRestartChance: (c: number) => void;
  setUnfamiliarWordSlowdown: (enabled: boolean) => void;
  setAutoPause: (enabled: boolean) => void;
  setAutoPauseInterval: (m: number) => void;
  setAutoPauseDuration: (m: number) => void;
  setSubstitution: (enabled: boolean) => void;
  setInsertion: (enabled: boolean) => void;
  setOmission: (enabled: boolean) => void;
  setDoubleLetter: (enabled: boolean) => void;
  setTransposition: (enabled: boolean) => void;
  setWrongCaps: (enabled: boolean) => void;
  setCorrectionEnabled: (enabled: boolean) => void;
  setCorrectionSpeed: (speed: CorrectionSpeed) => void;
  setOverBackspaceChance: (c: number) => void;
  setPunctuationPauses: (enabled: boolean) => void;
  setParagraphPause: (p: ParagraphPause) => void;
  toggleSettingsPanel: () => void;
}

export const useSettingsStore = create<SettingsStore>()(
  persist(
    (set) => ({
      activePersonality: "steady-eddie",
      customWpm: 65,
      interactionMode: "main",
      advancedOpen: false,
      settingsPanelOpen: true,
      errorRate: 0.03,
      speedVariation: 0.18,

      thinkingPausesEnabled: true,
      thinkingFrequency: 0.05,
      thinkingPreset: "Normal",
      fatigueEnabled: false,
      burstMode: false,
      rolloverEnabled: false,
      microCorrectionsEnabled: true,
      microCorrectionChance: 0.03,
      secondThoughtsEnabled: true,
      secondThoughtsChance: 0.08,
      synonymChance: 0.50,
      errorClusteringEnabled: true,

      // New behaviors — off by default
      wordSubstitutionEnabled: false,
      wordSubstitutionChance: 0.05,
      hesitationBackspaceEnabled: false,
      hesitationBackspaceChance: 0.01,
      midWordPauseEnabled: false,
      midWordPauseChance: 0.01,
      sentenceRestartEnabled: false,
      sentenceRestartChance: 0.005,
      unfamiliarWordSlowdown: false,
      autoPauseEnabled: false,
      autoPauseInterval: 30,
      autoPauseDuration: 5,

      substitutionEnabled: true,
      insertionEnabled: true,
      omissionEnabled: true,
      doubleLetterEnabled: true,
      transpositionEnabled: true,
      wrongCapsEnabled: false,

      correctionEnabled: true,
      correctionSpeed: "Quick",
      overBackspaceChance: 0.12,

      punctuationPausesEnabled: true,
      paragraphPause: "Normal",

      periodPause: [800, 1200],
      commaPause: [400, 600],
      questionPause: [900, 1300],
      exclamationPause: [900, 1300],
      colonPause: [500, 700],
      semicolonPause: [450, 650],

      // Actions
      setPersonality: (id) => set({ activePersonality: id }),
      setWpm: (wpm) => set({ customWpm: wpm }),
      toggleAdvanced: () => set((s) => ({ advancedOpen: !s.advancedOpen })),
      setErrorRate: (rate) => set({ errorRate: rate }),
      setSpeedVariation: (v) => set({ speedVariation: v }),
      setThinkingPauses: (enabled) => set({ thinkingPausesEnabled: enabled }),
      setThinkingFrequency: (f) => set({ thinkingFrequency: f }),
      setThinkingPreset: (p) => set({ thinkingPreset: p }),
      setFatigue: (enabled) => set({ fatigueEnabled: enabled }),
      setBurstMode: (enabled) => set({ burstMode: enabled }),
      setRollover: (enabled) => set({ rolloverEnabled: enabled }),
      setMicroCorrections: (enabled) => set({ microCorrectionsEnabled: enabled }),
      setMicroCorrectionChance: (c) => set({ microCorrectionChance: c }),
      setSecondThoughts: (enabled) => set({ secondThoughtsEnabled: enabled }),
      setSecondThoughtsChance: (c) => set({ secondThoughtsChance: c }),
      setSynonymChance: (c) => set({ synonymChance: c }),
      setErrorClustering: (enabled) => set({ errorClusteringEnabled: enabled }),
      setWordSubstitution: (enabled) => set({ wordSubstitutionEnabled: enabled }),
      setWordSubstitutionChance: (c) => set({ wordSubstitutionChance: c }),
      setHesitationBackspace: (enabled) => set({ hesitationBackspaceEnabled: enabled }),
      setHesitationBackspaceChance: (c) => set({ hesitationBackspaceChance: c }),
      setMidWordPause: (enabled) => set({ midWordPauseEnabled: enabled }),
      setMidWordPauseChance: (c) => set({ midWordPauseChance: c }),
      setSentenceRestart: (enabled) => set({ sentenceRestartEnabled: enabled }),
      setSentenceRestartChance: (c) => set({ sentenceRestartChance: c }),
      setUnfamiliarWordSlowdown: (enabled) => set({ unfamiliarWordSlowdown: enabled }),
      setAutoPause: (enabled) => set({ autoPauseEnabled: enabled }),
      setAutoPauseInterval: (m) => set({ autoPauseInterval: m }),
      setAutoPauseDuration: (m) => set({ autoPauseDuration: m }),
      setSubstitution: (enabled) => set({ substitutionEnabled: enabled }),
      setInsertion: (enabled) => set({ insertionEnabled: enabled }),
      setOmission: (enabled) => set({ omissionEnabled: enabled }),
      setDoubleLetter: (enabled) => set({ doubleLetterEnabled: enabled }),
      setTransposition: (enabled) => set({ transpositionEnabled: enabled }),
      setWrongCaps: (enabled) => set({ wrongCapsEnabled: enabled }),
      setCorrectionEnabled: (enabled) => set({ correctionEnabled: enabled }),
      setCorrectionSpeed: (speed) => set({ correctionSpeed: speed }),
      setOverBackspaceChance: (c) => set({ overBackspaceChance: c }),
      setPunctuationPauses: (enabled) => set({ punctuationPausesEnabled: enabled }),
      setParagraphPause: (p) => set({ paragraphPause: p }),
      toggleSettingsPanel: () => set((s) => ({ settingsPanelOpen: !s.settingsPanelOpen })),
    }),
    {
      name: "lucid-typer-settings",
      partialize: (state) => {
        const { setPersonality, setWpm, toggleAdvanced, toggleSettingsPanel, setErrorRate, setSpeedVariation,
          setThinkingPauses, setThinkingFrequency, setThinkingPreset, setFatigue, setBurstMode,
          setRollover, setMicroCorrections, setMicroCorrectionChance, setSecondThoughts,
          setSecondThoughtsChance, setSynonymChance, setErrorClustering, setWordSubstitution,
          setWordSubstitutionChance, setHesitationBackspace, setHesitationBackspaceChance,
          setMidWordPause, setMidWordPauseChance, setSentenceRestart, setSentenceRestartChance,
          setUnfamiliarWordSlowdown, setAutoPause, setAutoPauseInterval, setAutoPauseDuration,
          setSubstitution, setInsertion, setOmission, setDoubleLetter, setTransposition,
          setWrongCaps, setCorrectionEnabled, setCorrectionSpeed, setOverBackspaceChance,
          setPunctuationPauses, setParagraphPause, ...rest } = state;
        return rest;
      },
    }
  )
);
