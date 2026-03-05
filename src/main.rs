use clap::{CommandFactory, Parser};
use cli::Cli;

mod cli;
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

    // It checks if a graphical interface is already running,
    // if xinit is installed, and if the app exists. The binary or app have
    // to be in the path.
    verify::run(&app_name);

    println!("yes {} {:?}", app_name, app_args);
}
