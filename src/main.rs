use color_eyre::Result;

use clap::{Parser, Subcommand};
use console::Style;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Day {
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25), required_unless_present = "all")]
        day: Option<u8>,

        #[arg(short, long)]
        all: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day { day, all } => {
            match day {
                Some(1) => DAYS[0](),
                Some(2) => DAYS[1](),
                Some(3) => DAYS[2](),
                Some(4) => DAYS[3](),
                Some(5) => DAYS[4](),
                Some(6) => DAYS[5](),
                Some(7) => DAYS[6](),
                Some(8) => DAYS[7](),
                Some(9) => DAYS[8](),
                Some(10) => DAYS[9](),
                Some(11) => DAYS[10](),
                Some(12) => DAYS[11](),
                Some(13) => DAYS[12](),
                Some(14) => DAYS[13](),
                Some(15) => DAYS[14](),
                Some(16) => DAYS[15](),
                Some(17) => DAYS[16](),
                Some(18) => DAYS[17](),
                Some(19) => DAYS[18](),
                Some(20) => DAYS[19](),
                Some(21) => DAYS[20](),
                Some(22) => DAYS[21](),
                Some(23) => DAYS[22](),
                Some(24) => DAYS[23](),
                Some(25) => DAYS[24](),
                None => {
                    for exec in DAYS {
                        exec()?;
                        println!("")
                    }
                    Ok(())
                }
                _ => panic!("You're evil"),
            }
        }
        _ => todo!(),
    }
}

type MainFunction = fn() -> Result<()>;

const DAYS: [MainFunction; 25] = [
    day01::main,
    day02::main,
    day03::main,
    day04::main,
    day05::main,
    day06::main,
    day07::main,
    day08::main,
    day09::main,
    day10::main,
    day11::main,
    day12::main,
    day13::main,
    day14::main,
    day15::main,
    day16::main,
    day17::main,
    day18::main,
    day19::main,
    day20::main,
    day21::main,
    day22::main,
    day23::main,
    day24::main,
    day25::main,
];
