use sha2::{Sha256, Digest};
use std::process::Command;

/// Get a hardware UUID and hash it for a stable device fingerprint.
fn get_hardware_id() -> Result<String, String> {
    let uuid = get_platform_uuid()?;
    let mut hasher = Sha256::new();
    hasher.update(uuid.as_bytes());
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

#[cfg(target_os = "macos")]
fn get_platform_uuid() -> Result<String, String> {
    let output = Command::new("ioreg")
        .args(["-rd1", "-c", "IOPlatformExpertDevice"])
        .output()
        .map_err(|e| format!("Failed to run ioreg: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("IOPlatformUUID") {
            if let Some(uuid) = line.split('"').nth(3) {
                return Ok(uuid.to_string());
            }
        }
    }

    Err("Could not find IOPlatformUUID".to_string())
}

#[cfg(target_os = "windows")]
fn get_platform_uuid() -> Result<String, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    // Try registry first — works on all Windows versions
    if let Ok(output) = Command::new("reg")
        .args(["query", r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography", "/v", "MachineGuid"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("MachineGuid") {
                if let Some(guid) = line.split_whitespace().last() {
                    return Ok(guid.to_string());
                }
            }
        }
    }

    // Fallback: try PowerShell
    if let Ok(output) = Command::new("powershell")
        .args(["-NoProfile", "-Command", "(Get-CimInstance -Class Win32_ComputerSystemProduct).UUID"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let trimmed = stdout.trim();
        if !trimmed.is_empty() && trimmed != "Error" {
            return Ok(trimmed.to_string());
        }
    }

    // Last resort: use COMPUTERNAME + username as a pseudo-ID
    let computer = std::env::var("COMPUTERNAME").unwrap_or_default();
    let user = std::env::var("USERNAME").unwrap_or_default();
    if !computer.is_empty() {
        return Ok(format!("{}-{}", computer, user));
    }

    Err("Could not determine Windows machine identity".to_string())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn get_platform_uuid() -> Result<String, String> {
    std::fs::read_to_string("/etc/machine-id")
        .or_else(|_| std::fs::read_to_string("/var/lib/dbus/machine-id"))
        .map(|s| s.trim().to_string())
        .map_err(|_| "Could not find machine ID".to_string())
}

fn get_device_name() -> String {
    get_platform_name()
}

#[cfg(target_os = "macos")]
fn get_platform_name() -> String {
    Command::new("scutil")
        .args(["--get", "ComputerName"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown Mac".to_string())
}

#[cfg(target_os = "windows")]
fn get_platform_name() -> String {
    std::env::var("COMPUTERNAME")
        .unwrap_or_else(|_| "Unknown PC".to_string())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn get_platform_name() -> String {
    Command::new("hostname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown Device".to_string())
}

#[tauri::command]
pub fn get_device_fingerprint() -> Result<(String, String), String> {
    let device_id = get_hardware_id()?;
    let device_name = get_device_name();
    Ok((device_id, device_name))
}
