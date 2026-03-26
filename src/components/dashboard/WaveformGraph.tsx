import { useRef, useEffect } from "react";
import { useEngineStore } from "../../stores/engine-store";

export function WaveformGraph() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { waveformData, status, targetWpm } = useEngineStore();
  const animationRef = useRef<number>(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const draw = () => {
      const dpr = window.devicePixelRatio || 1;
      const rect = canvas.getBoundingClientRect();
      canvas.width = rect.width * dpr;
      canvas.height = rect.height * dpr;
      ctx.scale(dpr, dpr);

      const w = rect.width;
      const h = rect.height;

      ctx.clearRect(0, 0, w, h);

      // Subtle grid
      ctx.strokeStyle = "rgba(255, 255, 255, 0.02)";
      ctx.lineWidth = 1;
      for (let y = 0; y < h; y += 20) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(w, y);
        ctx.stroke();
      }

      if (waveformData.length < 2) {
        animationRef.current = requestAnimationFrame(draw);
        return;
      }

      const maxWpm = Math.max(targetWpm * 1.5, ...waveformData.map((d) => d.wpm)) + 10;
      const minWpm = 0;

      // Target line (stays white/dim for contrast)
      const targetY = h - ((targetWpm - minWpm) / (maxWpm - minWpm)) * h;
      ctx.strokeStyle = "rgba(255, 255, 255, 0.06)";
      ctx.setLineDash([3, 3]);
      ctx.lineWidth = 1;
      ctx.shadowColor = "transparent";
      ctx.shadowBlur = 0;
      ctx.beginPath();
      ctx.moveTo(0, targetY);
      ctx.lineTo(w, targetY);
      ctx.stroke();
      ctx.setLineDash([]);

      // WPM line — green with glow
      const points = waveformData.slice(-100);
      const stepX = w / Math.max(points.length - 1, 1);

      ctx.strokeStyle = "rgba(52, 211, 153, 0.6)";
      ctx.lineWidth = 1.5;
      ctx.lineJoin = "round";
      ctx.lineCap = "round";
      ctx.shadowColor = "rgba(52, 211, 153, 0.4)";
      ctx.shadowBlur = 8;

      ctx.beginPath();
      points.forEach((point, i) => {
        const x = i * stepX;
        const y = h - ((point.wpm - minWpm) / (maxWpm - minWpm)) * h;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      });
      ctx.stroke();

      // Reset shadow for fill
      ctx.shadowColor = "transparent";
      ctx.shadowBlur = 0;

      // Fill under — green gradient
      const gradient = ctx.createLinearGradient(0, 0, 0, h);
      gradient.addColorStop(0, "rgba(52, 211, 153, 0.06)");
      gradient.addColorStop(1, "rgba(52, 211, 153, 0)");
      ctx.fillStyle = gradient;
      ctx.lineTo((points.length - 1) * stepX, h);
      ctx.lineTo(0, h);
      ctx.closePath();
      ctx.fill();

      animationRef.current = requestAnimationFrame(draw);
    };

    animationRef.current = requestAnimationFrame(draw);
    return () => cancelAnimationFrame(animationRef.current);
  }, [waveformData, status, targetWpm]);

  return <canvas ref={canvasRef} className="w-full h-full rounded" />;
}
