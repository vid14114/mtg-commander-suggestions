use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use mtg_commander_suggestions::{commander_suggestions, import_oracle};

#[derive(Parser)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ImportOracle { 
        #[arg(short, long)]
        remove_old: bool
    },
    ImportTags { 
        #[arg(short, long)]
        remove_old: bool
    },
    Suggest { csv_path: PathBuf },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let arguments = Arguments::parse();
    match arguments.command {
        Commands::ImportOracle { remove_old } => import_oracle(remove_old).await,
        Commands::ImportTags { remove_old } => todo!(),
        Commands::Suggest { csv_path } => {
            let commander_compatible_cards = commander_suggestions(csv_path).await;
            for (commander, compatible_cards) in &commander_compatible_cards {
                println!("{}", commander.name);
                for (keyword, cards) in compatible_cards {
                    println!("\t{keyword}");
                    for card in cards {
                        println!("\t\t{}", card.name);
                    }
                }
            }
        }
    }

    Ok(())
}
