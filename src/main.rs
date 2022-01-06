mod card_utils;
mod commander;
mod storage;

use std::{collections::HashMap, path::PathBuf};

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

fn filter_commanders(cards: &[Card]) -> Vec<Card> {
    cards
        .iter()
        .filter(|card| card.type_line.contains("Legendary"))
        .filter(|card| card.type_line.contains("Creature"))
        .map(|commander| commander.to_owned())
        .collect()
}

fn find_compatible_cards(
    commander_keywords: Vec<(Card, Vec<String>)>,
    collection: &[Card],
) -> Vec<(Card, HashMap<String, Vec<Card>>)> {
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
) -> HashMap<String, Vec<Card>> {
    let mut keywords_cards = HashMap::new();
    collection
        .iter()
        .filter(|card| {
            extract_card_colors(card)
                .iter()
                .all(|color| commander.color_identity.contains(color))
        })
        .filter_map(|card| {
            let keyword = keywords.iter().find(|&keyword| {
                card.type_line.contains(keyword) || extract_oracle_text(card).contains(keyword)
            });
            keyword.map(|keyword| (keyword.to_owned(), card.to_owned()))
        })
        .for_each(|(keyword, card)| {
            keywords_cards
                .entry(keyword)
                .or_insert_with(Vec::new)
                .push(card)
        });
    keywords_cards
}
