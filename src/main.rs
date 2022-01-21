use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use mtg_commander_suggestions::commander_suggestions;

#[derive(Parser)]
struct Arguments {
    csv_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let arguments = Arguments::parse();

    let commander_compatible_cards = commander_suggestions(arguments.csv_path).await;
    for (commander, compatible_cards) in &commander_compatible_cards {
        println!("{}", commander.name);
        for (keyword, cards) in compatible_cards {
            println!("\t{keyword}");
            for card in cards {
                println!("\t\t{}", card.name);
            }
        }
    }

    Ok(())
}
