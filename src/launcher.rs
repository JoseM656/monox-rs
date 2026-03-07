use crate::session::Session;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn launch(app_name: &str, app_args: &[&String], session: &Session) {
    let xinitrc = generate_xinitrc(app_name, app_args, session);
    start_x(&xinitrc);
}

fn generate_xinitrc(app_name: &str, app_args: &[&String], session: &Session) -> NamedTempFile {
    let mut file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(_) => {
            merror!("Failed to create temp xinitrc");
            std::process::exit(1);
        }
    };

    let args: Vec<&str> = app_args.iter().map(|s| s.as_str()).collect();
    let args_str = args.join(" ");

    let xrandr_cmd = match &session.resolution {
        Some(res) => format!("xrandr --mode {}", res),
        None => "xrandr --auto".to_string(),
    };

    let content = format!(
        "#!/bin/sh\n{}\nxsetroot -solid black\nexec {} {}\n",
        xrandr_cmd, app_name, args_str
    );

    if let Err(_) = file.write_all(content.as_bytes()) {
        merror!("Failed to write xinitrc");
        std::process::exit(1);
    }

    if let Err(_) = fs::set_permissions(file.path(), fs::Permissions::from_mode(0o755)) {
        merror!("Failed to set xinitrc permissions");
        std::process::exit(1);
    }

    file
}

fn start_x(xinitrc: &NamedTempFile) {
    launching!("Starting X11...");
    if let Err(_) = Command::new("startx").arg(xinitrc.path()).status() {
        merror!("Failed to start X11");
        std::process::exit(1);
    }
}
