use scryfall::{Card, Catalog};

use crate::card_utils::extract_oracle_text;

pub fn extract_catalogued_keywords(commanders: Vec<Card>) -> Vec<(Card, Vec<String>)> {
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
