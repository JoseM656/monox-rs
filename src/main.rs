use clap::{CommandFactory, Parser};
use cli::Cli;

#[macro_use]
mod printer;

mod cli;
mod launcher;
mod session;
mod verify;

fn main() {
    // process input
    let cli = Cli::parse();

    let app_name = match cli.app.first() {
        Some(name) => name,
        None => {
            merror!("Invalid argument");
            Cli::command().print_help().unwrap();
            std::process::exit(0);
        }
    };

    let app_args: Vec<&String> = cli.app.iter().skip(1).collect();

    // --------------------------------
    // Verification and lauch app zone
    // --------------------------------

    // Step 1
    checking!("Starting verification...");
    // It checks if a graphical interface is already running,
    // if xinit is installed, and if the app exists. The binary or app have
    // to be in the path.
    verify::run(&app_name);

    // Yes, yes I know a variable declaration in line 41 but this is secuencial!!!
    checking!("Starting critical services...");
    // Step 2
    let session = session::Session::start();

    // Step 3
    launching!("Generating xinitrc file of: {}", &app_name);
    // Launch creating a temporaly file of xinitrc, in that file puts the app and flags.
    launcher::launch(app_name, &app_args);
}
