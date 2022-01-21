use mtg_commander_suggestions::{commander_suggestions, storage::get_card_collection};
use std::{
    fs::File,
    path::PathBuf,
};

use scryfall::Card;

#[tokio::test]
async fn recognise_card() {
    setup_database().await;
    let result = commander_suggestions(PathBuf::from("./tests/minimal-collection.csv")).await;
    assert_eq!(result.len(), 1);
    let (_, keywords) = &result[0];
    assert!(keywords.keys().any(|keyword| keyword.eq("Elf")));
}

async fn setup_database() {
    let file = File::open("tests/abomination-of-llanowar.json").unwrap();
    let card: Card = serde_json::from_reader(file).unwrap();
    let collection = get_card_collection();
    collection.await.insert_one(card, None).await.unwrap();
}
