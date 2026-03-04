use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();

    let app_name = cli.app.first().expect("No app provided");
    let app_args: Vec<&String> = cli.app.iter().skip(1).collect();

    println!("{} {:?}", app_name, app_args);
}
