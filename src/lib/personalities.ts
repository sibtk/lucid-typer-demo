export interface Personality {
  id: string;
  name: string;
  description: string;
  icon: string;
  glowColor: "cyan" | "purple" | "green" | "amber" | "pink" | "red";
  wpm: number;
  errorRate: number;
  speedVariation: number;
  burstMode: boolean;
  fatigueResistance: number;
  thinkingFrequency: number;
  rolloverEnabled: boolean;
  rolloverChance: number;
}

export const personalities: Personality[] = [
  {
    id: "speed-demon",
    name: "Speed Demon",
    description: "Blazing fast with barely any mistakes. Pure velocity.",
    icon: "\u26A1",
    glowColor: "cyan",
    wpm: 120,
    errorRate: 0.015,
    speedVariation: 0.09,
    burstMode: false,
    fatigueResistance: 0.8,
    thinkingFrequency: 0.03,
    rolloverEnabled: true,
    rolloverChance: 0.65,
  },
  {
    id: "careful-typist",
    name: "Careful Typist",
    description: "Slow and deliberate. Almost never makes a mistake.",
    icon: "\uD83C\uDFAF",
    glowColor: "green",
    wpm: 55,
    errorRate: 0.008,
    speedVariation: 0.12,
    burstMode: false,
    fatigueResistance: 0.9,
    thinkingFrequency: 0.06,
    rolloverEnabled: false,
    rolloverChance: 0,
  },
  {
    id: "sloppy-fast",
    name: "Sloppy Fast",
    description: "Quick but messy. Types fast, corrects often.",
    icon: "\uD83D\uDCA8",
    glowColor: "red",
    wpm: 95,
    errorRate: 0.06,
    speedVariation: 0.14,
    burstMode: false,
    fatigueResistance: 0.6,
    thinkingFrequency: 0.03,
    rolloverEnabled: true,
    rolloverChance: 0.5,
  },
  {
    id: "hunt-and-pecker",
    name: "Hunt & Pecker",
    description: "Searches for each key. Slow but endearing.",
    icon: "\uD83D\uDD0D",
    glowColor: "amber",
    wpm: 28,
    errorRate: 0.07,
    speedVariation: 0.28,
    burstMode: false,
    fatigueResistance: 0.5,
    thinkingFrequency: 0.08,
    rolloverEnabled: false,
    rolloverChance: 0,
  },
  {
    id: "burst-typer",
    name: "Burst Typer",
    description: "Rapid bursts of typing with pauses to think.",
    icon: "\uD83D\uDCA5",
    glowColor: "purple",
    wpm: 85,
    errorRate: 0.025,
    speedVariation: 0.16,
    burstMode: true,
    fatigueResistance: 0.7,
    thinkingFrequency: 0.04,
    rolloverEnabled: true,
    rolloverChance: 0.4,
  },
  {
    id: "steady-eddie",
    name: "Steady Eddie",
    description: "Consistent and predictable. The average typist.",
    icon: "\u2696\uFE0F",
    glowColor: "cyan",
    wpm: 65,
    errorRate: 0.03,
    speedVariation: 0.18,
    burstMode: false,
    fatigueResistance: 0.7,
    thinkingFrequency: 0.05,
    rolloverEnabled: false,
    rolloverChance: 0,
  },
  {
    id: "fatigued-worker",
    name: "Fatigued Worker",
    description: "Starts strong but degrades quickly. Monday energy.",
    icon: "\uD83D\uDE34",
    glowColor: "amber",
    wpm: 75,
    errorRate: 0.025,
    speedVariation: 0.15,
    burstMode: false,
    fatigueResistance: 0.2,
    thinkingFrequency: 0.05,
    rolloverEnabled: false,
    rolloverChance: 0,
  },
  {
    id: "nervous-typist",
    name: "Nervous Typist",
    description: "Erratic speed, lots of rewrites and corrections.",
    icon: "\uD83D\uDE30",
    glowColor: "pink",
    wpm: 58,
    errorRate: 0.045,
    speedVariation: 0.25,
    burstMode: false,
    fatigueResistance: 0.4,
    thinkingFrequency: 0.07,
    rolloverEnabled: false,
    rolloverChance: 0,
  },
];

export function getPersonality(id: string): Personality {
  return personalities.find((p) => p.id === id) || personalities[5]; // default to Steady Eddie
}
