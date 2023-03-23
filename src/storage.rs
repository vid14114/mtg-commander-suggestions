use core::fmt;
use std::path::PathBuf;

use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use scryfall::{
    bulk::oracle_cards,
    search::{query::Query, Search},
    Card,
};
use serde::{Deserialize, Serialize};

use crate::scryfall_tags::fetch_tags;

pub async fn update_oracle(remove_old: bool) {
    let collection = get_card_collection().await;
    if remove_old {
        collection
            .drop(None)
            .await
            .expect("Mongodb drop card collection");
    }

    // Fetch and insert oracle cards if database is not populated
    if collection
        .estimated_document_count(None)
        .await
        .expect("Mongodb estimated count")
        < 1
    {
        let cards = oracle_cards().expect("Fetch oracle cards");
        for card in cards {
            collection
                .insert_one(card.expect("Fetch card"), None)
                .await
                .expect("Insert card");
        }
        println!(
            "Imported about {} cards",
            collection
                .estimated_document_count(None)
                .await
                .expect("Mongo estimated document count")
        );
    } else {
        println!("Existing collection detected. Not updating!")
    }
}

pub async fn update_tags(remove_old: bool) {
    let tag_collection = get_tag_collection().await;
    if remove_old {
        tag_collection
            .drop(None)
            .await
            .expect("Mongodb drop tag collection");
    }

    // Fetch and insert tags if database is not populated
    if tag_collection
        .estimated_document_count(None)
        .await
        .expect("Mongodb estimated count")
        < 1
    {
        let tags = fetch_tags(None).await.expect("Fetch scryfall tags");
        for tag in tags {
            let cards = search_by_oracletag(&tag).unwrap_or_default();
            println!("{} results", cards.len());
            tag_collection.insert_one(Tag { tag, cards }, None).await.expect("Insert tags");
        }
    } else {
        println!("Existing collection detected. Not updating!")
    }
}

pub fn search_by_oracletag(tag: &str) -> Result<Vec<Card>, scryfall::Error> {
    print!("Searching for oracletag: {}; ", tag);
    let query = Query::Custom(format!("{}\"{}\"", "oracletag:", tag));
    let cards = query.search_all()?;
    Ok(cards)
}

pub async fn read_deckbox_collection(csv_path: PathBuf) -> Vec<Card> {
    let mut rdr = csv::Reader::from_path(csv_path).expect("CSV Reader path");
    let cards = get_card_collection().await;
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

pub async fn get_card_collection() -> Collection<Card> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("Mongo parse client options");
    let client = Client::with_options(client_options).expect("Mongo create client");
    let db = client.database("oracle_cards");
    db.collection::<Card>("cards")
}

pub async fn get_tag_collection() -> Collection<Tag> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("Mongo parse client options");
    let client = Client::with_options(client_options).expect("Mongo create client");
    let db = client.database("oracle_cards");
    db.collection::<Tag>("tags")
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CsvCard {
    name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
    cards: Vec<Card>
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}
