use color_eyre::Result;

use clap::{Parser};

mod day01;
mod day02;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    day: i8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Running day {}", cli.day);

    match cli.day {
        1 => day01::main(),
        2 => day02::main(),
        _ => todo!(),
    }
}
