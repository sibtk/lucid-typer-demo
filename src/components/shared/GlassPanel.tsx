import React from "react";

type GlowColor = "cyan" | "purple" | "green" | "amber" | "pink" | "red" | "none";

interface GlassPanelProps {
  children: React.ReactNode;
  className?: string;
  glowColor?: GlowColor;
  hover?: boolean;
  onClick?: () => void;
}

const glowMap: Record<string, string> = {
  cyan: "glow-cyan",
  purple: "glow-purple",
  green: "glow-green",
  amber: "glow-amber",
  pink: "glow-pink",
  red: "glow-red",
  none: "",
};

export function GlassPanel({ children, className = "", glowColor = "none", hover = false, onClick }: GlassPanelProps) {
  return (
    <div
      onClick={onClick}
      className={`glass ${glowMap[glowColor]} ${hover ? "hover:border-glass-hover transition-all duration-200 cursor-pointer hover:scale-[1.01]" : ""} ${className}`}
    >
      {children}
    </div>
  );
}
