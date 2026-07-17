use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;

use chxss::cli::commands::Command;
use chxss::cli::io;
use chxss::pgn::PgnParser;
use chxss::pgn::write_game;

#[derive(Parser)]
#[command(name = "chxss", version, about = "Chess data processing toolkit")]
struct Cli {
    /// Input file (reads from stdin if omitted)
    #[arg(short = 'i', long = "input", global = true)]
    input: Option<PathBuf>,

    /// Output file (writes to stdout if omitted)
    #[arg(short = 'o', long = "output", global = true)]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let reader = io::open_input(cli.input.as_deref())?;
    let buf_reader = io::buf_reader(reader);

    let mut writer = io::open_output(cli.output.as_deref())?;

    match cli.command {
        Command::MinElo { min, max } => {
            let games = PgnParser::new(buf_reader).collect::<Vec<_>>();
            let filtered = chxss::tools::pgn::filter::min_elo(games, min, max);
            for game in filtered {
                let game = game?;
                write_game(&mut writer, &game)?;
            }
        }
        Command::MinPly { min } => {
            let games = PgnParser::new(buf_reader).collect::<Vec<_>>();
            let filtered = chxss::tools::pgn::filter::min_ply(games, min);
            for game in filtered {
                let game = game?;
                write_game(&mut writer, &game)?;
            }
        }
    }

    Ok(())
}
