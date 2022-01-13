use std::fs::{canonicalize, File};

use assert_cmd::{crate_name, Command};
use mtg_commander_suggestions::storage::get_card_collection;
use predicates::str::contains;
use scryfall::Card;

#[tokio::test]
async fn recognise_card() {
    setup_database().await;
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.current_dir(canonicalize("./tests").unwrap())
        .arg("minimal-collection.csv")
        .assert()
        .success()
        .stdout(contains("Elf"));
}

async fn setup_database() {
    let file = File::open("tests/abomination-of-llanowar.json").unwrap();
    let card: Card = serde_json::from_reader(file).unwrap();
    let collection = get_card_collection();
    collection.await.insert_one(card, None).await.unwrap();
}
