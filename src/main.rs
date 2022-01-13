use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use mtg_commander_suggestions::{
    commander::extract_catalogued_keywords,
    filter_commanders, find_compatible_cards,
    storage::{read_deckbox_collection, update_oracle},
};

#[derive(Parser)]
struct Arguments {
    csv_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let arguments = Arguments::parse();

    let cards_database = update_oracle().await;
    println!(
        "Imported about {} cards",
        cards_database
            .estimated_document_count(None)
            .await
            .expect("Mongo estimated document count")
    );

    let recognised_cards = read_deckbox_collection(cards_database, arguments.csv_path).await;
    println!(
        "Recognised {} cards from collection",
        &recognised_cards.len()
    );

    let commanders = filter_commanders(&recognised_cards);
    println!("Found {} possible commanders", commanders.len());

    let commander_keywords = extract_catalogued_keywords(commanders);
    for (commander, keywords) in &commander_keywords {
        println!("{}; {:#?}", commander.name, keywords);
    }
    let mut commander_compatible_cards =
        find_compatible_cards(commander_keywords, &recognised_cards);
    commander_compatible_cards.sort_unstable_by(|(_, cards1), (_, cards2)| {
        cards2
            .values()
            .flatten()
            .count()
            .cmp(&cards1.values().flatten().count())
    });
    for (commander, compatible_cards) in &commander_compatible_cards {
        println!("{}", commander.name);
        for (keyword, cards) in compatible_cards {
            println!("\t{}", keyword);
            for card in cards {
                println!("\t\t{}", card.name);
            }
        }
    }
    Ok(())
}
