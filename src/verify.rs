pub fn run(app: &str) {
    checking!("Cheking X11...");
    check_display();
    done!();

    checking!("Checking xinit installation...");
    check_xinit();
    done!();

    checking!("Checking '{}' installation...", app);
    check_app(app);
    done!();
}

fn check_display() {
    if std::env::var("DISPLAY").is_ok() {
        merror!("X11 is already running. Try to run in a tty1 envieroment");
        std::process::exit(1);
    }
}

fn check_xinit() {
    if which::which("xinit").is_err() {
        merror!("xinit not found, install xorg-xinit");
        std::process::exit(1);
    }
}

fn check_app(app: &str) {
    if which::which(app).is_err() {
        merror!("'{}' not found in PATH", app);
        std::process::exit(1);
    }
}
