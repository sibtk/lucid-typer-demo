const API_BASE = import.meta.env.VITE_API_BASE_URL || "http://localhost:3000";

export interface PipelineEvent {
  type: string;
  message?: string;
  score?: number;
  text?: string;
  pass?: number;
  passNumber?: number;
  passType?: string;
  maxPasses?: number;
  totalPasses?: number;
  overallScore?: number;
  flaggedCount?: number;
  totalSentences?: number;
  humanizedText?: string;
  finalScore?: number;
  creditsUsed?: number;
}

export async function humanizeText(
  text: string,
  mode: "light" | "balanced" | "aggressive",
  onEvent: (event: PipelineEvent) => void,
  signal?: AbortSignal,
): Promise<void> {
  const res = await fetch(`${API_BASE}/api/humanize/stream`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ text, mode, accessToken: "" }),
    signal,
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: "Request failed" }));
    throw new Error(err.error || `HTTP ${res.status}`);
  }

  const reader = res.body?.getReader();
  if (!reader) throw new Error("No response body");

  const decoder = new TextDecoder();
  let buffer = "";

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;

    buffer += decoder.decode(value, { stream: true });
    const lines = buffer.split("\n");
    buffer = lines.pop() || "";

    for (const line of lines) {
      if (line.startsWith("data: ")) {
        try {
          const event = JSON.parse(line.slice(6)) as PipelineEvent;
          onEvent(event);
        } catch {
          // skip malformed events
        }
      }
    }
  }
}
