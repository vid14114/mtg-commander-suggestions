mod card_utils;
mod commander;
mod storage;

use std::path::PathBuf;

use card_utils::{extract_card_colors, extract_oracle_text};
use clap::Parser;
use color_eyre::eyre::Result;
use scryfall::Card;

use crate::{
    commander::extract_catalogued_keywords,
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
    commander_compatible_cards
        .sort_unstable_by(|(_, cards1), (_, cards2)| cards2.len().cmp(&cards1.len()));
    for (commander, compatible_cards) in &commander_compatible_cards {
        println!(
            "{}; {} compatible cards",
            commander.name,
            compatible_cards.len()
        );
    }
    Ok(())
}

fn filter_commanders(cards: &[Card]) -> Vec<Card> {
    cards
        .iter()
        .filter(|card| card.type_line.contains("Legendary Creature"))
        .map(|commander| commander.to_owned())
        .collect()
}

fn find_compatible_cards(
    commander_keywords: Vec<(Card, Vec<String>)>,
    collection: &[Card],
) -> Vec<(Card, Vec<Card>)> {
    commander_keywords
        .iter()
        .map(|(commander, keywords)| {
            (
                commander.to_owned(),
                match_colors_and_keywords(commander, keywords, collection),
            )
        })
        .collect()
}

fn match_colors_and_keywords(
    commander: &Card,
    keywords: &[String],
    collection: &[Card],
) -> Vec<Card> {
    collection
        .iter()
        .filter(|card| {
            extract_card_colors(card)
                .iter()
                .all(|color| commander.color_identity.contains(color))
        })
        .filter(|card| {
            keywords.iter().any(|keyword| {
                card.type_line.contains(keyword) || extract_oracle_text(card).contains(keyword)
            })
        })
        .map(|card| card.to_owned())
        .collect()
}
