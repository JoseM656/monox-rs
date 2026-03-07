use std::process::Child;
use std::process::Command;

pub struct Session {
    pub resolution: Option<String>,
    processes: Vec<Child>, // dbus, pipewire — session kills them on exit
}

impl Session {
    pub fn start() -> Self {
        let mut processes = vec![];

        setup_dbus();

        if let Some(child) = start_audio() {
            processes.push(child);
        }

        Session {
            resolution: detect_resolution(),
            processes,
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        for proc in &mut self.processes {
            proc.kill().ok();
        }
    }
}

fn setup_dbus() {
    launching!("Setting up dbus...");

    if which::which("dbus-launch").is_err() {
        merror!("dbus-launch not found, some apps may not work");
        return; // If doesn't find dbus continuous, some apps can work anyway.
    }

    let output = match std::process::Command::new("dbus-launch")
        .arg("--sh-syntax")
        .output()
    {
        Ok(o) => o,
        Err(_) => {
            merror!("Failed to run dbus-launch");
            return;
        }
    };

    // parse "DBUS_SESSION_BUS_ADDRESS='unix:path=...'; export DBUS_SESSION_BUS_ADDRESS;"
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some((key, val)) = parse_dbus_var(line) {
            // safe: single-threaded, called before any other threads are spawned
            unsafe {
                std::env::set_var(key, val);
            }
        }
    }
    done!();
}

fn parse_dbus_var(line: &str) -> Option<(&str, &str)> {
    // format: KEY='value'; export KEY;
    let line = line.trim();
    if !line.contains('=') {
        return None;
    }
    let (key, rest) = line.split_once('=')?;
    let val = rest.trim_matches(|c| c == '\'' || c == ';').trim();
    // Filter only the relevant variables
    if key.starts_with("DBUS_") {
        Some((key, val))
    } else {
        None
    }
}

// Search for audio services, there are the most commun ones
// but may I can put more.
fn start_audio() -> Option<Child> {
    launching!("Setting up audio...");

    if which::which("pipewire").is_ok() {
        let child = Command::new("pipewire").spawn().ok()?;
        done!();
        return Some(child);
    }

    if which::which("pulseaudio").is_ok() {
        let child = Command::new("pulseaudio").arg("--start").spawn().ok()?;
        done!();
        return Some(child);
    }

    merror!("No audio service found, continuing without audio");
    None
}

fn detect_resolution() -> Option<String> {
    // This is the only way I now to ask the resolution whitout x11
    let modes = std::fs::read_to_string("/sys/class/drm/card0-HDMI-A-1/modes").ok()?;
    modes.lines().next().map(|s| s.to_string())
}
