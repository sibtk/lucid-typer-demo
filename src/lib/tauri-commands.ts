import { invoke } from "@tauri-apps/api/core";

// The EngineConfig type mirrors the Rust EngineConfig struct.
// Using Record<string, unknown> for the full config since it's built dynamically in ActionBar.
export type EngineConfig = Record<string, unknown>;

export interface EstimateResult {
  total_seconds: number;
  typing_seconds: number;
  pause_seconds: number;
  estimated_errors: number;
}

export async function startTyping(text: string, config?: EngineConfig, mode?: string, countdown?: number) {
  return invoke("start_typing", { text, config, mode, countdown });
}

export async function stopTyping() {
  return invoke("stop_typing");
}

export async function pauseTyping() {
  return invoke("pause_typing");
}

export async function resumeTyping() {
  return invoke("resume_typing");
}

export async function getEngineState(): Promise<string> {
  return invoke("get_engine_state");
}

export async function previewTyping(text: string, config?: EngineConfig) {
  return invoke("preview_typing", { text, config });
}

export async function estimateTypingTime(text: string, config?: EngineConfig): Promise<EstimateResult> {
  return invoke("estimate_typing_time", { text, config });
}

export async function getDeviceFingerprint(): Promise<[string, string]> {
  return invoke("get_device_fingerprint");
}
