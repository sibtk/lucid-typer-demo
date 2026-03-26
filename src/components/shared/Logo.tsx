interface LogoProps {
  size?: number;
  className?: string;
}

export function Logo({ size = 24, className }: LogoProps) {
  const id = `logo-${size}`;
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 48 48"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      <defs>
        <filter id={`${id}-blur`}>
          <feGaussianBlur stdDeviation="2" />
        </filter>
      </defs>

      {/* Glow layer — blurred duplicate behind */}
      <g filter={`url(#${id}-blur)`}>
        <line x1="24" y1="4" x2="4" y2="16" stroke="#34d399" strokeWidth="3" strokeOpacity="0.3" />
        <line x1="24" y1="4" x2="44" y2="16" stroke="#34d399" strokeWidth="3" strokeOpacity="0.3" />
        <line x1="4" y1="16" x2="44" y2="16" stroke="#34d399" strokeWidth="3" strokeOpacity="0.3" />
        <line x1="4" y1="16" x2="24" y2="44" stroke="#34d399" strokeWidth="3" strokeOpacity="0.3" />
        <line x1="44" y1="16" x2="24" y2="44" stroke="#34d399" strokeWidth="3" strokeOpacity="0.3" />
      </g>

      {/* Gem — y-axis symmetric, x-axis asymmetric (short crown, long pavilion) */}
      {/* Top point */}
      <line x1="24" y1="4" x2="4" y2="16" stroke="#34d399" strokeWidth="1" />
      <line x1="24" y1="4" x2="44" y2="16" stroke="#34d399" strokeWidth="1" />

      {/* Girdle */}
      <line x1="4" y1="16" x2="44" y2="16" stroke="#34d399" strokeWidth="1" />

      {/* Crown inner facets */}
      <line x1="24" y1="4" x2="16" y2="16" stroke="#34d399" strokeWidth="1" strokeOpacity="0.7" />
      <line x1="24" y1="4" x2="32" y2="16" stroke="#34d399" strokeWidth="1" strokeOpacity="0.7" />

      {/* Bottom point — much longer than top */}
      <line x1="4" y1="16" x2="24" y2="44" stroke="#34d399" strokeWidth="1" />
      <line x1="44" y1="16" x2="24" y2="44" stroke="#34d399" strokeWidth="1" />

      {/* Pavilion inner facets */}
      <line x1="16" y1="16" x2="24" y2="44" stroke="#34d399" strokeWidth="1" strokeOpacity="0.7" />
      <line x1="32" y1="16" x2="24" y2="44" stroke="#34d399" strokeWidth="1" strokeOpacity="0.7" />
      <line x1="24" y1="16" x2="24" y2="44" stroke="#34d399" strokeWidth="1" strokeOpacity="0.7" />
    </svg>
  );
}
