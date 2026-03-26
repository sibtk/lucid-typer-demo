import React from "react";

type GlowColor = "cyan" | "purple" | "green" | "amber" | "pink" | "red";

interface GlowButtonProps {
  children: React.ReactNode;
  color?: GlowColor;
  variant?: "solid" | "outline" | "ghost";
  size?: "sm" | "md" | "lg";
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
}

const colorStyles: Record<GlowColor, { solid: string; outline: string; ghost: string }> = {
  cyan: {
    solid: "bg-glow-cyan/20 border-glow-cyan/40 text-glow-cyan hover:bg-glow-cyan/30 hover:shadow-[0_0_20px_rgba(0,212,255,0.3)]",
    outline: "border-glow-cyan/40 text-glow-cyan hover:bg-glow-cyan/10 hover:shadow-[0_0_15px_rgba(0,212,255,0.2)]",
    ghost: "text-glow-cyan hover:bg-glow-cyan/10",
  },
  purple: {
    solid: "bg-glow-purple/20 border-glow-purple/40 text-glow-purple hover:bg-glow-purple/30 hover:shadow-[0_0_20px_rgba(168,85,247,0.3)]",
    outline: "border-glow-purple/40 text-glow-purple hover:bg-glow-purple/10 hover:shadow-[0_0_15px_rgba(168,85,247,0.2)]",
    ghost: "text-glow-purple hover:bg-glow-purple/10",
  },
  green: {
    solid: "bg-glow-green/20 border-glow-green/40 text-glow-green hover:bg-glow-green/30 hover:shadow-[0_0_20px_rgba(34,197,94,0.3)]",
    outline: "border-glow-green/40 text-glow-green hover:bg-glow-green/10 hover:shadow-[0_0_15px_rgba(34,197,94,0.2)]",
    ghost: "text-glow-green hover:bg-glow-green/10",
  },
  amber: {
    solid: "bg-glow-amber/20 border-glow-amber/40 text-glow-amber hover:bg-glow-amber/30 hover:shadow-[0_0_20px_rgba(245,158,11,0.3)]",
    outline: "border-glow-amber/40 text-glow-amber hover:bg-glow-amber/10 hover:shadow-[0_0_15px_rgba(245,158,11,0.2)]",
    ghost: "text-glow-amber hover:bg-glow-amber/10",
  },
  pink: {
    solid: "bg-glow-pink/20 border-glow-pink/40 text-glow-pink hover:bg-glow-pink/30 hover:shadow-[0_0_20px_rgba(236,72,153,0.3)]",
    outline: "border-glow-pink/40 text-glow-pink hover:bg-glow-pink/10 hover:shadow-[0_0_15px_rgba(236,72,153,0.2)]",
    ghost: "text-glow-pink hover:bg-glow-pink/10",
  },
  red: {
    solid: "bg-glow-red/20 border-glow-red/40 text-glow-red hover:bg-glow-red/30 hover:shadow-[0_0_20px_rgba(239,68,68,0.3)]",
    outline: "border-glow-red/40 text-glow-red hover:bg-glow-red/10 hover:shadow-[0_0_15px_rgba(239,68,68,0.2)]",
    ghost: "text-glow-red hover:bg-glow-red/10",
  },
};

const sizeStyles = {
  sm: "px-3 py-1.5 text-xs rounded-lg",
  md: "px-4 py-2 text-sm rounded-lg",
  lg: "px-6 py-3 text-base rounded-xl",
};

export function GlowButton({
  children,
  color = "cyan",
  variant = "solid",
  size = "md",
  onClick,
  disabled = false,
  className = "",
}: GlowButtonProps) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`
        border font-medium transition-all duration-200
        ${sizeStyles[size]}
        ${colorStyles[color][variant]}
        ${disabled ? "opacity-40 cursor-not-allowed" : "cursor-pointer active:scale-95"}
        ${className}
      `}
    >
      {children}
    </button>
  );
}
