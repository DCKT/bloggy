use bloggy::build::build;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    build: Option<BuildCommands>,
}

#[derive(Subcommand)]
enum BuildCommands {
    Build {
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.build {
        Some(BuildCommands::Build { path }) => {
            let working_path = match path.as_deref() {
                None => "./",
                Some(p) => p,
            };

            build(working_path);
        }
        None => (),
    }
}
