use color_eyre::Result;

use clap::{Parser, Subcommand};
use console::Style;

mod day01;
mod day02;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day { day: i8 },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day { day } => {
            let bold = Style::new().bold();
            let green = bold.clone().green();
            println!(
                "{} {} {}...",
                bold.apply_to("Running"),
                green.apply_to("Day"),
                green.apply_to(day)
            );
            println!("");
            match day {
                1 => day01::main(),
                2 => day02::main(),
                _ => todo!(),
            }
        }
        _ => todo!(),
    }
}
