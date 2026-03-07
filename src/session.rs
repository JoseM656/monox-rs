use std::process::Child;
use std::process::Command;

pub struct Session {
    pub resolution: Option<(String, String)>,
    processes: Vec<Child>,
}

impl Session {
    pub fn start() -> Self {
        let mut processes = vec![];

        setup_dbus();
        processes.extend(start_audio());

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
        return;
    }

    let output = match Command::new("dbus-launch").arg("--sh-syntax").output() {
        Ok(o) => o,
        Err(_) => {
            merror!("Failed to run dbus-launch");
            return;
        }
    };

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
    let line = line.trim();
    if !line.contains('=') {
        return None;
    }
    let (key, rest) = line.split_once('=')?;
    let val = rest.trim_matches(|c| c == '\'' || c == ';').trim();
    if key.starts_with("DBUS_") {
        Some((key, val))
    } else {
        None
    }
}

fn start_audio() -> Vec<Child> {
    launching!("Setting up audio...");
    let mut procs = vec![];

    if which::which("pipewire").is_ok() {
        match Command::new("pipewire").spawn() {
            Ok(child) => procs.push(child),
            Err(_) => {
                merror!("Failed to spawn pipewire");
                return procs;
            }
        }

        if which::which("wireplumber").is_ok() {
            match Command::new("wireplumber").spawn() {
                Ok(child) => procs.push(child),
                Err(_) => merror!("Failed to spawn wireplumber, audio may not work correctly"),
            }
        } else {
            merror!("wireplumber not found, audio may not work correctly");
        }

        done!();
        return procs;
    }

    if which::which("pulseaudio").is_ok() {
        match Command::new("pulseaudio").arg("--start").spawn() {
            Ok(child) => procs.push(child),
            Err(_) => merror!("Failed to spawn pulseaudio"),
        }
        done!();
        return procs;
    }

    merror!("No audio service found, continuing without audio");
    procs
}

// There is many places where this information can be....
fn detect_resolution() -> Option<(String, String)> {
    // (conector, resolucion)
    let drm_path = std::path::Path::new("/sys/class/drm");

    for entry in std::fs::read_dir(drm_path).ok()?.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !name.contains('-') {
            continue;
        }

        let modes_path = entry.path().join("modes");
        if let Ok(modes) = std::fs::read_to_string(&modes_path) {
            if let Some(res) = modes.lines().next() {
                // card0-LVDS-1 → LVDS-1
                let connector = name.splitn(2, '-').nth(1)?.to_string();
                checking!("Resolution detected: {} on {}", res, connector);
                return Some((connector, res.to_string()));
            }
        }
    }

    merror!("Could not detect resolution, falling back to xrandr --auto");
    None
}
