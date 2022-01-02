use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use scryfall::{
    bulk::oracle_cards,
    card::{CardFace, Color},
    Card, Catalog,
};
use serde::Deserialize;

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
            "{}; {} compatible_cards",
            commander.name,
            compatible_cards.len()
        );
    }
    Ok(())
}

async fn update_oracle() -> Collection<Card> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("Mongo parse client options");
    let client = Client::with_options(client_options).expect("Mongo create client");
    let db = client.database("oracle_cards");
    let collection = db.collection::<Card>("cards");

    // Fetch and insert oracle cards if database is not populated
    if collection
        .estimated_document_count(None)
        .await
        .expect("Mongodb estimated count")
        < 24600
    {
        //currently there are about 24683 cards in oracle bulk data
        let cards = oracle_cards().expect("Fetch oracle cards");
        for card in cards {
            collection
                .insert_one(card.expect("Fetch card"), None)
                .await
                .expect("Insert card");
        }
    }
    collection
}

async fn read_deckbox_collection(cards: Collection<Card>, csv_path: PathBuf) -> Vec<Card> {
    let mut rdr = csv::Reader::from_path(csv_path).expect("CSV Reader path");
    let mut recognised_cards = vec![];
    for line in rdr.deserialize() {
        let csv_card: CsvCard = line.expect("CSV line");
        match cards
            .find_one(doc! {"name": &csv_card.name}, None)
            .await
            .expect("Mongo find card")
        {
            Some(card) => {
                if !recognised_cards.contains(&card) {
                    recognised_cards.push(card)
                }
            }
            None => println!("{} not found!", &csv_card.name),
        }
    }
    recognised_cards
}

fn filter_commanders(cards: &[Card]) -> Vec<Card> {
    cards
        .iter()
        .filter(|card| card.type_line.contains("Legendary Creature"))
        .map(|s| s.to_owned())
        .collect()
}

fn extract_catalogued_keywords(commanders: Vec<Card>) -> Vec<(Card, Vec<String>)> {
    let catalogs = vec![
        Catalog::creature_types()
            .expect("Catalog creature types")
            .data,
        Catalog::planeswalker_types()
            .expect("Catalog planeswalker types")
            .data,
        Catalog::land_types().expect("Catalog land types").data,
        Catalog::artifact_types()
            .expect("Catalog artifact types")
            .data,
        Catalog::enchantment_types()
            .expect("Catalog enchantment types")
            .data,
        Catalog::spell_types().expect("Catalog spell types").data,
        Catalog::keyword_actions()
            .expect("Catalog keyword actions")
            .data,
        Catalog::ability_words()
            .expect("Catalog ability words")
            .data,
        Catalog::keyword_abilities()
            .expect("Catalog keyword abilities")
            .data,
    ];
    commanders
        .iter()
        .map(|commander| (commander.to_owned(), find_keywords(commander, &catalogs)))
        .collect()
}

fn find_keywords(commander: &Card, catalogs: &[Vec<String>]) -> Vec<String> {
    let commander_card_text = extract_oracle_text(commander);

    let found_keywords = catalogs
        .iter()
        .flatten()
        .filter(|keyword| {
            commander_card_text
                .to_lowercase()
                .contains(keyword.to_lowercase().as_str())
        })
        .map(|s| s.to_owned())
        .collect();

    found_keywords
}

fn extract_oracle_text(card: &Card) -> String {
    match &card.oracle_text {
        Some(card_text) => card_text.to_owned(),
        None => extract_multiple_faces(card)
            .iter()
            .map(|face| {
                face.oracle_text
                    .as_ref()
                    .expect("Multiple faces oracle text")
            })
            .reduce(|accum, iter| {
                accum.to_owned().push_str(iter);
                accum
            })
            .expect("Multiple faces reduce")
            .to_string(),
    }
}

fn extract_card_colors(card: &Card) -> Vec<Color> {
    match &card.colors {
        Some(colors) => colors.to_owned(),
        None => extract_multiple_faces(card)
            .iter()
            .map(|face| {
                face.colors
                    .as_ref()
                    .expect("Multiple faces colors")
                    .to_owned()
            })
            .flatten()
            .collect(),
    }
}

fn extract_multiple_faces(card: &Card) -> &Vec<CardFace> {
    card.card_faces.as_ref().expect("Card with multiple faces")
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
                .all(|c| commander.color_identity.contains(c))
        })
        .filter(|card| {
            keywords.iter().any(|keyword| {
                card.type_line.contains(keyword) || extract_oracle_text(card).contains(keyword)
            })
        })
        .map(|s| s.to_owned())
        .collect()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CsvCard {
    name: String,
}

#[derive(Parser)]
struct Arguments {
    csv_path: PathBuf,
}
