use std::path::PathBuf;

use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use scryfall::{bulk::oracle_cards, Card};
use serde::Deserialize;

pub async fn update_oracle() -> Collection<Card> {
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

pub async fn read_deckbox_collection(cards: Collection<Card>, csv_path: PathBuf) -> Vec<Card> {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CsvCard {
    name: String,
}
