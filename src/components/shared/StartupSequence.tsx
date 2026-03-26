import { useState, useEffect } from "react";
import { Logo } from "./Logo";

const TITLE = "Lucid Typer";
const CHAR_STAGGER = 55;
const GLOW_PHASE = 700;
const TEXT_PHASE = 1600;
const FADE_PHASE = 2200;

export function StartupSequence({ onComplete }: { onComplete: () => void }) {
  const [phase, setPhase] = useState<"glow" | "text" | "fade">("glow");

  useEffect(() => {
    // Skip on HMR
    if (sessionStorage.getItem("lucid-booted")) {
      onComplete();
      return;
    }

    const t1 = setTimeout(() => setPhase("text"), GLOW_PHASE);
    const t2 = setTimeout(() => setPhase("fade"), TEXT_PHASE);
    const t3 = setTimeout(() => {
      sessionStorage.setItem("lucid-booted", "1");
      onComplete();
    }, FADE_PHASE);

    return () => {
      clearTimeout(t1);
      clearTimeout(t2);
      clearTimeout(t3);
    };
  }, [onComplete]);

  return (
    <div
      className="fixed inset-0 z-[100] flex flex-col items-center justify-center bg-black"
      style={phase === "fade" ? { animation: "startup-fade-out 500ms ease forwards" } : undefined}
    >
      {/* Radial glow */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          background: "radial-gradient(circle at 50% 50%, rgba(52,211,153,0.08) 0%, transparent 60%)",
          animation: "startup-glow 400ms ease-out forwards",
        }}
      />

      {/* Logo — fades in during glow phase */}
      <div
        style={{
          animation: "startup-glow 700ms ease-out forwards",
          transform: "scale(0.9)",
          transition: "transform 600ms cubic-bezier(0.16, 1, 0.3, 1)",
          ...(phase !== "glow" ? { transform: "scale(1)" } : {}),
        }}
      >
        <Logo size={72} />
      </div>

      {/* Title — character by character */}
      {(phase === "text" || phase === "fade") && (
        <div className="relative mt-4">
          <h1 className="text-[28px] font-semibold tracking-tight">
            {TITLE.split("").map((char, i) => (
              <span
                key={i}
                className="inline-block"
                style={{
                  animation: `char-reveal 200ms cubic-bezier(0.16, 1, 0.3, 1) both`,
                  animationDelay: `${i * CHAR_STAGGER}ms`,
                  color: "#e4e4e7",
                }}
              >
                {char === " " ? "\u00A0" : char}
              </span>
            ))}
          </h1>
          <span
            className="block text-center text-[11px] font-mono text-white/30 mt-2"
            style={{
              animation: "char-reveal 300ms cubic-bezier(0.16, 1, 0.3, 1) both",
              animationDelay: "400ms",
            }}
          >
            v0.1
          </span>
        </div>
      )}
    </div>
  );
}
