use crate::printer::{print_event, MonoxEvent};

pub fn run(app: &str) {

    print_event(MonoxEvent::Checking("Checking X11...".to_string()));
    check_display();
    print_event(MonoxEvent::Done);

    print_event(MonoxEvent::Checking("Checking xinit installation...".to_string()));
    check_xinit();
    print_event(MonoxEvent::Done);

    print_event(MonoxEvent::Checking(format!("Checking '{}' installation...", app)));
    check_app(app);
    print_event(MonoxEvent::Done);
}

fn check_display() {
    if std::env::var("DISPLAY").is_ok() {
        print_event(MonoxEvent::Error("X11 is already running".to_string()));
        std::process::exit(1);
    }
}

fn check_xinit() {
    if which::which("xinit").is_err() {
        print_event(MonoxEvent::Error("xinit not found, install xorg-xini".to_string()));
        std::process::exit(1);
    }
}

fn check_app(app: &str) {
    if which::which(app).is_err() {
        print_event(MonoxEvent::Error(format!("'{}' not found in PATH", app)));
        std::process::exit(1);
    }
}
