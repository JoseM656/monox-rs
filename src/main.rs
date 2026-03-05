use clap::{CommandFactory, Parser};
use cli::Cli;

mod cli;
mod launcher;
mod printer;
mod verify;

use crate::printer::{MonoxEvent, print_event};

fn main() {
    // process input
    let cli = Cli::parse();

    let app_name = match cli.app.first() {
        Some(name) => name,
        None => {
            print_event(MonoxEvent::Error("Invalid argument".to_string()));
            Cli::command().print_help().unwrap();
            std::process::exit(0);
        }
    };

    let app_args: Vec<&String> = cli.app.iter().skip(1).collect();

    print_event(MonoxEvent::Checking("Starting verification...".to_string()));
    // It checks if a graphical interface is already running,
    // if xinit is installed, and if the app exists. The binary or app have
    // to be in the path.
    verify::run(&app_name);
    print_event(MonoxEvent::Launching("Finished verification.".to_string()));

    print_event(MonoxEvent::Launching(format!("Starting {}", &app_name)));
    // Launch creating a temporaly file of xinitrc, in that file puts the app and flags.
    launcher::launch(app_name, &app_args);
    
}
