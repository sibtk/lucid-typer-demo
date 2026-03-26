use tauri::{AppHandle, State};
use crate::engine::config::EngineConfig;
use crate::engine::core::{self, EngineState, SharedState, EstimateResult};

#[tauri::command]
pub async fn start_typing(
    text: String,
    config: Option<EngineConfig>,
    mode: Option<String>,
    countdown: Option<u32>,
    state: State<'_, SharedState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let config = config.unwrap_or_default();
    let shared = state.inner().clone();
    let inject_keys = mode.as_deref() != Some("preview");
    let countdown_seconds = countdown.unwrap_or(5);

    // Check if already running, otherwise reset to idle
    {
        let mut s = shared.lock().map_err(|e| e.to_string())?;
        if *s == EngineState::Running {
            return Err("Engine is already running".to_string());
        }
        *s = EngineState::Idle;
    }

    // Spawn the typing session in a background task
    tauri::async_runtime::spawn(async move {
        core::run_typing_session(text, config, shared, app_handle, inject_keys, countdown_seconds).await;
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_typing(state: State<'_, SharedState>) -> Result<(), String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    *s = EngineState::Cancelled;
    Ok(())
}

#[tauri::command]
pub async fn pause_typing(state: State<'_, SharedState>) -> Result<(), String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    if *s == EngineState::Running {
        *s = EngineState::Paused;
    }
    Ok(())
}

#[tauri::command]
pub async fn resume_typing(state: State<'_, SharedState>) -> Result<(), String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    if *s == EngineState::Paused {
        *s = EngineState::Running;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_engine_state(state: State<'_, SharedState>) -> Result<String, String> {
    let s = state.lock().map_err(|e| e.to_string())?;
    Ok(s.to_string())
}

#[tauri::command]
pub async fn preview_typing(
    text: String,
    config: Option<EngineConfig>,
    state: State<'_, SharedState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    // Preview mode — no key injection and no countdown
    start_typing(text, config, Some("preview".to_string()), Some(0), state, app_handle).await
}

#[tauri::command]
pub async fn estimate_typing_time(
    text: String,
    config: Option<EngineConfig>,
) -> Result<EstimateResult, String> {
    let config = config.unwrap_or_default();
    Ok(core::estimate_typing_time(&text, &config))
}
