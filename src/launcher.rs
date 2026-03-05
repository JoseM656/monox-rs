use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::NamedTempFile;
use crate::printer::{print_event, MonoxEvent};

pub fn launch(app_name: &str, app_args: &[&String]) {
    let xinitrc = generate_xinitrc(app_name, app_args);
    start_x(&xinitrc);
}

fn generate_xinitrc(app_name: &str, app_args: &[&String]) -> NamedTempFile {

    let mut file = match NamedTempFile::new() {

        Ok(f) => f,
        Err(_) => {
            print_event(MonoxEvent::Error("Failed to create temp xinitrc".to_string()));
            std::process::exit(1);
        }
    };

    let args: Vec<&str> = app_args.iter().map(|s| s.as_str()).collect();

    let args_str = args.join(" ");

    let content = format!(
        "#!/bin/sh\nxsetroot -solid black\nexec {} {}\n",
        app_name, args_str
    );

    if let Err(_) = file.write_all(content.as_bytes()) {

        print_event(MonoxEvent::Error("Failed to write xinitrc".to_string()));
        std::process::exit(1);
    }

    if let Err(_) = fs::set_permissions(file.path(), fs::Permissions::from_mode(0o755)) {

        print_event(MonoxEvent::Error("Failed to set xinitrc permissions".to_string()));
        std::process::exit(1);
    }

    file
}

fn start_x(xinitrc: &NamedTempFile) {

    print_event(MonoxEvent::Launching("Starting X11...".to_string()));

    if let Err(_) = Command::new("startx").arg(xinitrc.path()).status() {
        
        print_event(MonoxEvent::Error("Failed to start X11".to_string()));
        std::process::exit(1);
    }
}